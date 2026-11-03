param([string]$Stage = 'final')
$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName System.Drawing
$projectRoot = Split-Path $PSScriptRoot -Parent
$outputRoot = Join-Path $projectRoot 'artifacts/visual-qa'
$regions = @{
    recent = [System.Drawing.Rectangle]::new(306, 92, 1279, 900)
    images = [System.Drawing.Rectangle]::new(330, 170, 1228, 790)
    passwords = [System.Drawing.Rectangle]::new(807, 170, 752, 330)
    settings = [System.Drawing.Rectangle]::new(330, 210, 1190, 625)
}
foreach ($page in @('recent', 'images', 'passwords', 'settings')) {
    $reference = [System.Drawing.Bitmap]::new((Join-Path $projectRoot "reference/$page.png"))
    $implementationPath = Join-Path $outputRoot "$page-$Stage.png"
    $implementation = [System.Drawing.Bitmap]::new($implementationPath)
    if ($implementation.Width -ne 1586 -or $implementation.Height -ne 992) {
        throw "$page viewport is not 1586 x 992"
    }
    if ($implementation.RawFormat.Guid -eq [System.Drawing.Imaging.ImageFormat]::Jpeg.Guid) {
        Copy-Item -LiteralPath $implementationPath -Destination (Join-Path $outputRoot "$page-$Stage.capture.jpg")
        $decoded = [System.Drawing.Bitmap]::new($implementation)
        $implementation.Dispose()
        $decoded.Save($implementationPath, [System.Drawing.Imaging.ImageFormat]::Png)
        $decoded.Dispose()
        $implementation = [System.Drawing.Bitmap]::new($implementationPath)
        Write-Output "$page capture is JPEG-derived; PNG encoding does not restore lost detail"
    }
    $comparison = [System.Drawing.Bitmap]::new(3172, 992)
    $graphics = [System.Drawing.Graphics]::FromImage($comparison)
    $graphics.DrawImageUnscaled($reference, 0, 0)
    $graphics.DrawImageUnscaled($implementation, 1586, 0)
    $comparison.Save((Join-Path $outputRoot "$page-compare-$Stage.png"))
    $graphics.Dispose()
    $comparison.Dispose()

    $region = $regions[$page]
    $referenceCrop = $reference.Clone($region, $reference.PixelFormat)
    $implementationCrop = $implementation.Clone($region, $implementation.PixelFormat)
    $focused = [System.Drawing.Bitmap]::new($region.Width * 2, $region.Height)
    $graphics = [System.Drawing.Graphics]::FromImage($focused)
    $graphics.DrawImageUnscaled($referenceCrop, 0, 0)
    $graphics.DrawImageUnscaled($implementationCrop, $region.Width, 0)
    $focused.Save((Join-Path $outputRoot "focused-$page-$Stage.png"))
    $graphics.Dispose()
    $focused.Dispose()
    $referenceCrop.Dispose()
    $implementationCrop.Dispose()
    $reference.Dispose()
    $implementation.Dispose()
    Write-Output "$page comparison saved ($Stage)"
}
