//! Backlog 0538: a package only the app's tests import is told apart from one the app uses.

use super::*;

#[test]
fn a_package_only_the_tests_import_is_named_with_the_test() {
    let mut imports = Imports::default();
    imports.read(
        "python",
        "app/main.py",
        "from fastapi import FastAPI\nimport os, json\n",
    );
    imports.read(
        "python",
        "tests/test_api.py",
        "import httpx\nfrom fastapi.testclient import TestClient\n",
    );
    assert_eq!(
        imports.only_in_tests("Python", "httpx"),
        Some("tests/test_api.py")
    );
    // Imported by the app too, so it counts.
    assert_eq!(imports.only_in_tests("Python", "fastapi"), None);
    // Imported by nothing seen, so it counts: a server, a plugin, or a name imported otherwise.
    assert_eq!(imports.only_in_tests("Python", "uvicorn"), None);
    assert_eq!(imports.only_in_tests("Python", "python-dotenv"), None);
}

#[test]
fn javascript_imports_are_read_by_package() {
    let mut imports = Imports::default();
    imports.read(
        "typescript",
        "src/server.ts",
        "import express from 'express';\nconst x = require(\"@aws-sdk/client-s3\");\nimport './local';\n",
    );
    imports.read(
        "javascript",
        "src/app.test.js",
        "const request = require('supertest');\nimport { s3 } from '@aws-sdk/client-s3/dist';\nimport axios from \"axios\";\n",
    );
    assert_eq!(
        imports.only_in_tests("npm", "supertest"),
        Some("src/app.test.js")
    );
    assert_eq!(
        imports.only_in_tests("npm", "axios"),
        Some("src/app.test.js")
    );
    assert_eq!(imports.only_in_tests("npm", "@aws-sdk/client-s3"), None);
    assert_eq!(imports.only_in_tests("npm", "express"), None);
    // Another ecosystem is never set aside.
    assert_eq!(imports.only_in_tests("Go", "supertest"), None);
}

#[test]
fn python_names_are_compared_as_imports_write_them() {
    let mut imports = Imports::default();
    imports.read(
        "python",
        "tests/conftest.py",
        "import google_auth\nfrom .helpers import x\n",
    );
    assert_eq!(
        imports.only_in_tests("Python", "Google-Auth"),
        Some("tests/conftest.py")
    );
    // A relative import names no package.
    assert_eq!(imports.only_in_tests("Python", "helpers"), None);
}
