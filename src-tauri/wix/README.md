# How to use this garbage...

- Once you've included any DLL's referenced in the .INF files, run `drivers/re-pack-files.cmd`
- This will create a `InstallDriver.exe` deliverable. 
- Then make sure the `include_drivers.xml` path is correctly set to point to the new `InstallDriver.exe` file in the `tauri.conf.json` file.

That's it!