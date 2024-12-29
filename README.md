# Minecraft Mod Utilities
update mods given a location and a list of download links.

## Usage
- make a file called mmu_config.json in the same folder as the exe
- Example file
```json
{
  "mods": [
    {
      "name": "test",
      "location": "C:\\Users\\<USER>\\AppData\\Roaming\\PrismLauncher\\instances\\minecraft\\.minecraft\\mods",
      "mods": [
        {
          "name": "Sodium",
          "pattern": "sodium-fabric-*+mc1.21.jar",
          "download_link": "https://github.com/CaffeineMC/sodium"
        }
      ]
    }
  ]
}
```
- The pattern should be the name of the file with the version number replaced with an asterisk.

