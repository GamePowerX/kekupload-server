/*
* Created on Wed Jun 01 2022
*
* Copyright (c) 2022 KotwOSS
*/

table! {
    files (id) {
        id -> Bpchar,
        ext -> Varchar,
        hash -> Bpchar,
    }
}
