//! Backlog 0624, part 5: two locators for questions that had none. `locate.memcache` points the question on cleaning
//! what goes to memcache (V1.3.9) at the files that bring in a memcache client, and leaves an ordinary dictionary or
//! a cache of another kind alone; `locate.media-server` names the media server packages beside the WebRTC media
//! questions (V17.2.1 to V17.2.4), and says none only where none can be said.

use super::*;
use crate::sbom::{Component, Sbom, VersionSource};
use crate::test_support::scratch;

fn shipped() -> Locators {
    Locators::load(
        &std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/locators.json"),
    )
    .expect("the shipped locators load, every query compiled")
}

/// The places `locate.memcache` found in a folder holding one file, as `file:line`.
fn memcache(name: &str, text: &str) -> Vec<String> {
    let dir = scratch("");
    std::fs::write(dir.join(name), text).unwrap();
    let listing = sv_scan::files::Listing::of(&dir);
    // The setup worked: the file was listed as the app's own code, or finding nothing would mean nothing.
    assert_eq!(listing.app_files().count(), 1, "{name} was not listed");
    shipped()
        .find(&listing)
        .places
        .get("locate.memcache")
        .map(|places| {
            places
                .iter()
                .map(|p| format!("{}:{}", p.file, p.line))
                .collect()
        })
        .unwrap_or_default()
}

#[test]
fn python_memcache_clients_are_found_and_other_imports_are_not() {
    let places = memcache(
        "app.py",
        "from pymemcache.client.base import Client\n\
         import memcache\n\
         import pylibmc as mc\n\
         from flask_caching import Cache\n\
         import redis\n\
         cache = {}\n\
         cache.get(key)\n",
    );
    assert_eq!(places, ["app.py:1", "app.py:2", "app.py:3"]);
}

#[test]
fn javascript_memcache_clients_are_found_and_other_modules_are_not() {
    let places = memcache(
        "server.js",
        "const Memcached = require('memcached');\n\
         import memjs from \"memjs\";\n\
         const express = require('express');\n\
         import { LRUCache } from 'lru-cache';\n\
         cache.set(key, value);\n",
    );
    assert_eq!(places, ["server.js:1", "server.js:2"]);
}

fn media_server() -> PackageLocator {
    shipped()
        .packages()
        .iter()
        .find(|l| l.id == "locate.media-server")
        .cloned()
        .expect("the media server's package locator is shipped")
}

fn component(ecosystem: &str, name: &str, version: &str) -> Component {
    Component {
        name: name.to_owned(),
        version: version.to_owned(),
        ecosystem: ecosystem.to_owned(),
        source: VersionSource::Locked,
    }
}

#[test]
fn the_media_server_speaks_to_the_four_media_questions_and_names_its_package() {
    let l = media_server();
    assert_eq!(l.questions, ["V17.2.1", "V17.2.2", "V17.2.3", "V17.2.4"]);
    let found = l.find(&[
        component("npm", "express", "4.19.2"),
        component("npm", "mediasoup", "3.14.6"),
        component("Go", "github.com/pion/webrtc/v4", "4.0.1"),
    ]);
    assert_eq!(
        found,
        ["github.com/pion/webrtc/v4 4.0.1", "mediasoup 3.14.6"]
    );
}

#[test]
fn none_is_said_only_for_a_read_list_with_no_media_server() {
    let l = media_server();
    let sbom = |components: Vec<Component>| Sbom {
        components,
        ..Sbom::default()
    };
    let npm = vec!["npm".to_owned()];
    assert!(
        l.none.is_some(),
        "the setup's locator has a sentence for none"
    );
    assert!(l.says_none(&sbom(vec![component("npm", "express", "4.19.2")]), &npm));
    assert!(!l.says_none(
        &sbom(vec![
            component("npm", "express", "4.19.2"),
            component("npm", "mediasoup", "3.14.6"),
        ]),
        &npm
    ));
    // Its words do not claim there is no media at all: a hosted service has no package.
    assert!(l.none.unwrap().contains("hosted service"));
}
