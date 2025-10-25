$ErrorActionPreference = 'Stop'

$packageName = 'drop-compress-image'
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$version = '2.3.0'
$url64 = "https://github.com/logue/DropWebP/releases/download/v$version/Drop.Compress.Image_$($version)_x64_en-US.msi"

$packageArgs = @{
  packageName    = $packageName
  fileType       = 'msi'
  url64bit       = $url64
  softwareName   = 'Drop Compress Image*'
  checksum64     = '5D9B7F826569CAF66F97378DA8826B9DDCADA0D448F86C63FC08E9459F0B8252'  # Will be filled by build script
  checksumType64 = 'sha256'
  silentArgs     = "/qn /norestart /l*v `"$($env:TEMP)\$($packageName).$($env:chocolateyPackageVersion).MsiInstall.log`""
  validExitCodes = @(0, 3010, 1641)
}

Install-ChocolateyPackage @packageArgs
