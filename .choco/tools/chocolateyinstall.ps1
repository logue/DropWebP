$ErrorActionPreference = 'Stop'

$packageName = 'drop-compress-image'
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$version = '3.2.1'
$url64 = "https://github.com/logue/DropWebP/releases/download/$version/drop-compress-image_$($version)_x64_en-US.msi"
$packageArgs = @{
  packageName    = $packageName
  fileType       = 'msi'
  url64bit       = $url64
  softwareName   = 'Drop Compress Image*'
  checksum64     = '56D377AE85B032715ADD0BCC3E10BB61FBB1BD373D7C438E79BAECA6FEACA81E'  # Will be filled by build script
  checksumType64 = 'sha256'
  silentArgs     = "/qn /norestart /l*v `"$($env:TEMP)\$($packageName).$($env:chocolateyPackageVersion).MsiInstall.log`""
  validExitCodes = @(0, 3010, 1641)
}

Install-ChocolateyPackage @packageArgs
