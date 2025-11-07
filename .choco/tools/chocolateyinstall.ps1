$ErrorActionPreference = 'Stop'

$packageName = 'drop-compress-image'
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$version = '3.0.1'
$url64 = "https://github.com/logue/DropWebP/releases/download/v$version/Drop.Compress.Image_$($version)_x64_en-US.msi"

$packageArgs = @{
  packageName    = $packageName
  fileType       = 'msi'
  url64bit       = $url64
  softwareName   = 'Drop Compress Image*'
  checksum64     = 'C4F1F901900EFF2CE4CEE7BFF398A73A585E0E99DA766FEAC9D9A7264E04CA25'  # Will be filled by build script
  checksumType64 = 'sha256'
  silentArgs     = "/qn /norestart /l*v `"$($env:TEMP)\$($packageName).$($env:chocolateyPackageVersion).MsiInstall.log`""
  validExitCodes = @(0, 3010, 1641)
}

Install-ChocolateyPackage @packageArgs
