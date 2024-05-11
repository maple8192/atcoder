Param($Number)

cargo compete new $Number

if ( $? ) {
    Set-Location $Number

    New-Item test.ps1 -Value 'Param($P); cargo compete test $P' > $null
    New-Item sumbit.ps1 -Value 'Param($P); cargo compete submit $P' > $null
    New-Item notest.ps1 -Value 'Param($P); cargo compete submit $P --no-test' > $null
}
