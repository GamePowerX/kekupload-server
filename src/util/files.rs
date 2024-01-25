/*
* Created on Wed Jun 01 2022
*
* Copyright (c) 2024 GamePowerX
*/

pub fn get_filename(hash: String, ext: String) -> String {
    if ext.eq("none") {
        return hash;
    } else {
        return hash + "." + ext.as_str();
    }
}

pub fn is_image(extension: String) -> bool {
    extension.eq("png")
        || extension.eq("jpg")
        || extension.eq("jpeg")
        || extension.eq("ico")
        || extension.eq("gif")
        || extension.eq("bmp")
        || extension.eq("svg")
}
