$ErrorActionPreference = 'Stop'

$InstallDir = "$env:LOCALAPPDATA\Programs\gitp"
$BinaryName = "gitp.exe"
$BinaryPath = Join-Path $InstallDir $BinaryName

# Create installation directory
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null

# Direct download URL
$downloadUrl = "https://github.com/ayn0s/git-profile/releases/latest/download/gitp-windows-amd64.exe"

# Download the binary
Write-Host "Downloading gitp..."
Invoke-WebRequest -Uri $downloadUrl -OutFile $BinaryPath

# Add to PATH if not already present
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable(
        "Path",
        "$UserPath;$InstallDir",
        "User"
    )
}

Write-Host "gitp has been installed successfully!"
Write-Host "You may need to restart your terminal for the 'gitp' command to be available."