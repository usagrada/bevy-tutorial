from string import Template
plist = Template("""
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple Computer//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleDevelopmentRegion</key>
  <string>English</string>
  <key>CFBundleDisplayName</key>
  <string>bevy-tutorial</string>
  <key>CFBundleExecutable</key>
  <string>bevy-tutorial</string>
  <key>CFBundleIdentifier</key>
  <string>com.usagrada.exampleapplication</string>
  <key>CFBundleInfoDictionaryVersion</key>
  <string>6.0</string>
  <key>CFBundleName</key>
  <string>bevy-tutorial</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>CFBundleShortVersionString</key>
  <string>${version}</string>
  <key>CFBundleVersion</key>
  <string>${date}</string>
  <key>CSResourcesFileMapped</key>
  <true/>
  <key>LSApplicationCategoryType</key>
  <string>public.app-category.developer-tools</string>
  <key>LSRequiresCarbon</key>
  <true/>
  <key>NSHighResolutionCapable</key>
  <true/>
  <key>NSHumanReadableCopyright</key>
  <string>Copyright (c) usagrada 2021. All rights reserved.</string>
</dict>
</plist>
""")
version = "1.0.0"
import datetime

date=datetime.datetime.now()
# print(date)
print(plist.substitute(version=version, date=date))