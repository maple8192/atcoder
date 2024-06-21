Param($Number)

cargo compete new $Number

if ( $? ) {
    Set-Location $Number

    New-Item test.ps1 -Value 'Param($P); cargo compete test $P' > $null
    New-Item sub.ps1 -Value 'Param($P); cargo compete submit $P' > $null
    New-Item not.ps1 -Value 'Param($P); cargo compete submit $P --no-test' > $null
    $val = 'Param($P); cargo run --bin {0}-$P' -f "$Number"
    New-Item exe.ps1 -Value $val > $null
}
