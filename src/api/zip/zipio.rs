use std::path::Path;
use std::fs::{File, create_dir_all};
use std::io::{copy, Read, Write};
use walkdir::{WalkDir};

use log::*;

use zip::{ZipArchive, ZipWriter};
use zip::write::FileOptions;

pub struct Zip{}

impl Zip{
    pub fn extract(filepath:String, mut outpath:String) -> bool{
        if !outpath.ends_with("/") || !outpath.ends_with("\\"){
            outpath = format!("{}/", outpath);
        }
        let opath = Path::new(&outpath);
        let fname = Path::new(&filepath);
        let file = File::open(&fname);
        if file.is_ok(){
            let archive = ZipArchive::new(file.unwrap());
            if archive.is_ok(){
                let mut zarchive = archive.unwrap();
                for i in 0..zarchive.len(){
                    let file = zarchive.by_index(i);
                    if file.is_ok(){
                        let mut zfile = file.unwrap();
                        let outfilepath = opath.join(zfile.sanitized_name());

                        if zfile.name().ends_with("/"){
                            let res = create_dir_all(&outfilepath);
                            if res.is_err(){
                                //TODO clear existing entries inside the output folder
                                error!("Error extracting zip folder");
                                return false;
                            }
                        }else{
                            match outfilepath.parent(){
                                Some(folder) => {
                                    let res = create_dir_all(&folder);
                                    if res.is_err(){
                                        //TODO clear existing entries inside the output folder
                                        error!("Error extracting zip folder");
                                        return false;
                                    }
                                },
                                None => {}
                            }
                            let fileres = File::create(&outfilepath);
                            if fileres.is_ok(){
                                let wres = copy(&mut zfile, &mut fileres.unwrap());
                                if wres.is_err(){
                                    //TODO clear existing entries inside the output folder
                                    println!("Error extracting data from zip file");
                                    return false;
                                }

                                #[cfg(unix)]
                                {
                                    use std::os::unix::fs::PermissionsExt;
                                    use std::fs;
                                    if let Some(mode) = zfile.unix_mode(){
                                        if let Err(e) = fs::set_permissions(&outfilepath, fs::Permissions::from_mode(mode)){
                                            //TODO clear existing entries inside the output folder
                                            error!("Error setting permissions from zip file");
                                            return false;
                                        }
                                    }
                                }
                            }else{
                                //TODO clear existing entries inside the output folder
                                error!("Error extracting zip file");
                                return false;
                            }
                        }
                    }else{
                        error!("Error reading zip entry");
                        //TODO clear existing entries inside the output folder
                        return false;
                    }
                }
                return true;
            }else{
                error!("Error: Invalid zip archive");
                return false;
            }
        }else{
            error!("Error: Opening zip archive");
            return false;
        }
    }

    pub fn create(inpath:String, outfilepath:String) -> bool{
        let zpath = Path::new(&inpath);
        if !zpath.is_dir(){
            error!("Error: Invalid input - expected folder, found file");
            return false;
        }

        let opath = Path::new(&outfilepath);
        let zipfile = File::create(&opath);
        if zipfile.is_ok(){
            let walkdir = WalkDir::new(inpath.clone());
            let it = walkdir.into_iter().filter_map(|e| e.ok());

            let mut zip = ZipWriter::new(zipfile.unwrap());
            let options = FileOptions::default().compression_method(zip::CompressionMethod::Bzip2).unix_permissions(0o755);

            let mut buffer = Vec::new();
            for entry in it{
                let path = entry.path();
                let name_res = path.strip_prefix(Path::new(&inpath));
                if name_res.is_ok(){
                    let name = name_res.unwrap();
                    if path.is_file(){
                        let res = zip.start_file_from_path(name, options);
                        if res.is_ok(){
                            let fres = File::open(path);
                            if fres.is_ok(){
                                let mut f = fres.unwrap();
                                let read_res = f.read_to_end(&mut buffer);
                                if read_res.is_ok(){
                                    let zres = zip.write_all(&*buffer);
                                    if zres.is_ok(){
                                        buffer.clear();
                                    }else{
                                        error!("Error: adding file to zip file");
                                        return false;    
                                    }
                                }else{
                                    error!("Error: adding file to zip file");
                                    return false;    
                                }                                
                            }else{
                                error!("Error: adding file to zip file");
                                return false;    
                            }
                        }else{
                            error!("Error: adding file to zip file");
                            return false;
                        }
                    
                    }else if name.as_os_str().len() != 0{
                        // Only if not root! Avoids path spec / warning
                        // and mapname conversion failed error on unzip
                        let res = zip.add_directory_from_path(name, options);
                        if res.is_err(){
                            error!("Error: adding folder to zip file");
                            return false;
                        }
                    }
                }else{
                    error!("Could not create file path");
                    return false;
                }
            }
        }else{
            error!("Error: creating zip file");
            return false;
        }

        return true;
    }
}
