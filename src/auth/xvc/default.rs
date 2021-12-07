/*
 * Created on Mon Dec 06 2021
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use std::borrow::Cow;

use sha2::{Sha512, Digest};

use super::XVCHasher;

/// Default xvc hasher uses win32 platform client format.
/// 
/// Format: sha512("{first_seed}|{user_agent}|{second_seed}|{email}|{device_uuid}")
#[derive(Debug, Clone)]
pub struct Win32XVCHasher<'a> {
    first: Cow<'a, str>,
    second: Cow<'a, str>
}

impl<'a> Win32XVCHasher<'a> {
    pub const fn new(first: Cow<'a, str>, second: Cow<'a, str>) -> Self {
        Self {
            first,
            second
        }
    }
}

impl Win32XVCHasher<'static> {
    pub const fn new_const(first: &'static str, second: &'static str) -> Self {
        Self {
            first: Cow::Borrowed(first),
            second: Cow::Borrowed(second)
        }
    }
}

impl XVCHasher for Win32XVCHasher<'_> {
    fn full_xvc_hash(&self, device_uuid: &str, user_agent: &str, email: &str) -> Vec<u8> {
        let mut hasher = Sha512::new();

        hasher.update(&self.first as &str);
        hasher.update(&"|");
        hasher.update(user_agent);
        hasher.update(&"|");
        hasher.update(&self.second as &str);
        hasher.update(&"|");
        hasher.update(email);
        hasher.update(&"|");
        hasher.update(device_uuid);

        hasher.finalize().to_vec()
    }
}

/// Default xvc hasher uses android subdevice platform client format.
/// 
/// Format: sha512("{first_seed}|{email}|{second_seed}|{user_agent}|{third_seed}")
#[derive(Debug, Clone)]
pub struct AndroidSubXVCHasher<'a> {
    first: Cow<'a, str>,
    second: Cow<'a, str>,
    third: Cow<'a, str>
}

impl<'a> AndroidSubXVCHasher<'a> {
    pub const fn new(first: Cow<'a, str>, mid: Cow<'a, str>, third: Cow<'a, str>) -> Self {
        Self {
            first,
            second: mid,
            third
        }
    }
}

impl AndroidSubXVCHasher<'static> {
    pub const fn new_const(first: &'static str, second: &'static str, third: &'static str) -> Self {
        Self {
            first: Cow::Borrowed(first),
            second: Cow::Borrowed(second),
            third: Cow::Borrowed(third)
        }
    }
}

impl XVCHasher for AndroidSubXVCHasher<'_> {
    fn full_xvc_hash(&self, _: &str, user_agent: &str, email: &str) -> Vec<u8> {
        let mut hasher = Sha512::new();
        
        hasher.update(&self.first as &str);
        hasher.update(&"|");
        hasher.update(user_agent);
        hasher.update(&"|");
        hasher.update(&self.second as &str);
        hasher.update(&"|");
        hasher.update(email);
        hasher.update(&"|");
        hasher.update(&self.third as &str);

        hasher.finalize().to_vec()
    }
}
