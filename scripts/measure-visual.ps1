param([string]$Stage = 'final')
Add-Type -AssemblyName System.Drawing
$projectRoot = Split-Path $PSScriptRoot -Parent
$regions = @(
    @{Page='recent'; Name='brand'; Box=@(25,43,190,51)},
    @{Page='recent'; Name='nav'; Box=@(70,135,120,35)},
    @{Page='recent'; Name='collection'; Box=@(68,480,155,35)},
    @{Page='recent'; Name='column-title'; Box=@(335,109,140,33)},
    @{Page='recent'; Name='heading-line'; Box=@(814,180,550,34)},
    @{Page='recent'; Name='content-line'; Box=@(831,416,540,27)},
    @{Page='passwords'; Name='row-title'; Box=@(339,184,150,28)},
    @{Page='passwords'; Name='field-title'; Box=@(852,224,250,32)},
    @{Page='settings'; Name='capture-title'; Box=@(337,225,260,29)},
    @{Page='settings'; Name='danger-description'; Box=@(362,779,660,25)},
    @{Page='images'; Name='page-title'; Box=@(337,42,200,42)},
    @{Page='images'; Name='image-meta'; Box=@(1161,601,120,33)},
    @{Page='images'; Name='image-meta-value'; Box=@(1420,601,133,33)},
    @{Page='recent'; Name='search'; Box=@(394,32,250,32)},
    @{Page='recent'; Name='copy-selected'; Box=@(1425,342,126,32)},
    @{Page='recent'; Name='content-meta'; Box=@(830,727,225,32)},
    @{Page='passwords'; Name='password-label'; Box=@(835,178,85,30)},
    @{Page='passwords'; Name='masked-row'; Box=@(338,232,150,25)},
    @{Page='passwords'; Name='username-row'; Box=@(338,211,180,22)},
    @{Page='settings'; Name='danger-title'; Box=@(360,688,140,35)},
    @{Page='settings'; Name='website-button'; Box=@(1314,565,130,35)}
)
foreach ($region in $regions) {
    foreach ($version in @('reference', $Stage)) {
        $file = if ($version -eq 'reference') { "reference/$($region.Page).png" } else { "artifacts/visual-qa/$($region.Page)-$Stage.png" }
        $bitmap = [System.Drawing.Bitmap]::new((Join-Path $projectRoot $file))
        $box = $region.Box
        $left = 9999; $top = 9999; $right = 0; $bottom = 0
        for ($y=$box[1]; $y -lt $box[1]+$box[3]; $y++) {
            for ($x=$box[0]; $x -lt $box[0]+$box[2]; $x++) {
                $pixel = $bitmap.GetPixel($x,$y)
                if (($pixel.R+$pixel.G+$pixel.B)/3 -lt 145) {
                    $left=[Math]::Min($left,$x); $right=[Math]::Max($right,$x)
                    $top=[Math]::Min($top,$y); $bottom=[Math]::Max($bottom,$y)
                }
            }
        }
        [PSCustomObject]@{Region=$region.Name; Version=$version; X=$left; Y=$top; Width=$right-$left+1; Height=$bottom-$top+1}
        $bitmap.Dispose()
    }
}
