<#
VERSJA 1.25.0
----------------------------------------------------------------------------------------
|  UWAGA LOGICZNA PRZEZ ZŁĄ IMPLEMENTACJE ABY UZYSKAĆ TEŻ PUSTE FOLDERY, NALEŻY UŻYĆ   |                                         |
|  WZORCA `-pathPatterns "*", "!temp/","!.git/", "!target/", "!src-tauri/gen/"`        |
|  LOGICZNIE POWINIEN TO BYĆ PONIŻSZY ALE POMIJA PUSTE FOLDERY Z POWODU BŁĘDNEJ LOGIKI |                                   |
|  WZORZEC `-pathPatterns "**/*", "!temp/","!.git/", "!target/", "!src-tauri/gen/"`    |
----------------------------------------------------------------------------------------

UŻYCIE:
    .\plot-fs.ps1 -pathPatterns "*", "!*.png","!temp/","!.git/", "!target/", "!src-tauri/gen/", "target/release/*.exe" -sortStrategy B -sortDirFirst
#>

# ==============================================================================
# 1. ZMIENNE WEJŚCIOWE
# ==============================================================================

[CmdletBinding()]
param (
    [Parameter(Position = 0)] [string]$pathLocation = '.',
    [Parameter(Position = 1)] [string[]]$pathPatterns = @("*"),
    [Parameter(Mandatory=$false)] [string[]]$sortPriority = @("Cargo.lock", "deno.lock", "tauri.conf.json", "Cargo.toml", "deno.jsonc", "deno.json", "package.json"),
    [Parameter(Mandatory=$false)] [ValidateSet("A", "B", "C")] [string]$sortStrategy = "A",
    [Parameter(Mandatory=$false)] [ValidateRange(1, 10)] [int] $sortExtDepth = 4,
    [Parameter(Mandatory=$false)] [switch]$sortDirFirst,
    [switch]$DebugMode
)

# ==============================================================================
# 2. STYLIZACJA
# ==============================================================================

function Get-TreeStyle {
    # Konfiguracja "szkieletu" drzewa
    return [PSCustomObject]@{
        Color      = "DarkGray" # Kolor linii
        Vertical   = "│"        # Pionowa kreska (dla priorytetów)
        BranchLast = "└── "     # Ostatni element
        BranchMid  = "├── "     # Środkowy element
        IndentLast = "    "     # Wcięcie puste (gdy rodzic był ostatni)
        IndentMid  = "│   "     # Wcięcie z kreską (gdy rodzic ma rodzeństwo)
    }
}

function Get-DirStyle {
    # Konfiguracja wyglądu folderów
    return [PSCustomObject]@{
        Icon      = "📂 "
        IconColor = "Yellow"
        Suffix    = "/"
        TextColor = "Cyan"
    }
}

function Get-FileStyle {
    param ($Item)
    $Style = @{ Icon = "📄 "; Color = "White" }

    if ($Item.FullName -match "[\\/](capabilities|permissions)[\\/]") {
        $Style.Icon = "⚙️  "; $Style.Color = "White"
        return $Style
    }

    switch -Regex ($Item.Name) {
        "^(Cargo\.lock|deno\.lock)$" { $Style.Icon = "🔒 "; $Style.Color = "White" }
        "^(Cargo\.toml|deno\.jsonc?|config\.toml|settings\.json|tauri\.conf\.json|\.gitignore)$" {  $Style.Icon = "⚙️  "; $Style.Color = "White"  }
        "\.ps1$" { $Style.Icon = "▶️  "; $Style.Color = "White" }
        "\.exe$" { $Style.Icon = "📦 "; $Style.Color = "White" }
        "\.(png|svg|jpg|jpeg|ico|icns)$" { $Style.Icon = "🖼️  "; $Style.Color = "Gray" }
        "\.md$"  { $Style.Icon = "📖 "; $Style.Color = "White" }
        "\.rs$"  { $Style.Icon = "🦀 "; $Style.Color = "White" }
        "\.ts$"  { $Style.Icon = "🦕 "; $Style.Color = "White" }
        "^build.*\.(rs|ts)$" { $Style.Icon = "🏗️  "; $Style.Color = "White" }
    }
    return $Style
}

# ==============================================================================
# 3. LOGIKA SORTOWANIA
# ==============================================================================

function Get-SortKey {
    param ($Item)
    
    $TypeKey = if ($sortDirFirst -and $Item.PSIsContainer) { 0 } elseif ($sortDirFirst) { 1 } else { 0 }
    
    if ($Item.PSIsContainer) { return ,@($TypeKey, 200, $Item.Name) }

    $idx = [array]::IndexOf($sortPriority, $Item.Name)
    $BasePriority = if ($idx -ne -1) { $idx } else { 100 }

    if ($sortStrategy -eq "A") { return ,@($TypeKey, $BasePriority, $Item.Name) }

    $Parts = $Item.Name.Split('.')
    if ($sortStrategy -eq "B") {
        $Ext = if ($Parts.Count -gt 1) { $Parts[-1] } else { "" }
        return ,@($TypeKey, $BasePriority, $Ext, $Item.Name)
    }

    if ($sortStrategy -eq "C") {
        $Keys = @($TypeKey, $BasePriority)
        for ($i = 1; $i -le $sortExtDepth; $i++) {
            $Keys += if ($Parts.Count -ge $i) { $Parts[-$i] } else { "" }
        }
        $RemCount = $Parts.Count - $sortExtDepth
        $Keys += if ($RemCount -gt 0) { [string]::Join('.', $Parts[0..($RemCount-1)]) } else { "" }
        return ,$Keys
    }
}

# ==============================================================================
# 4. RYSOWANIE DRZEWA
# ==============================================================================

function Write-FileTree {
    param ([string]$CurrentPath, [string]$Indent = "")

    # Pobieramy style raz, aby używać ich w pętli
    $TreeStyle = Get-TreeStyle
    $DirStyle  = Get-DirStyle

    $Items = Get-ChildItem -Path $CurrentPath -Force -ErrorAction SilentlyContinue | 
             Where-Object { $allowedPaths.Contains($_.FullName) }

    $SortedItems = $Items | Sort-Object { (Get-SortKey $_)[0] }, 
                                        { (Get-SortKey $_)[1] }, 
                                        { (Get-SortKey $_)[2] }, 
                                        { (Get-SortKey $_)[3] }, 
                                        { (Get-SortKey $_)[4] }

    $Count = $SortedItems.Count
    $LastWasPriority = $false

    for ($i = 0; $i -lt $Count; $i++) {
        $Item = $SortedItems[$i]
        $IsLast = ($i -eq ($Count - 1))
        
        $IsPriority = $sortPriority -contains $Item.Name
        
        # --- SEPARATORY PRIORYTETÓW ---
        if ($i -eq 0 -and $IsPriority) { 
            Write-Host ($Indent + $TreeStyle.Vertical) -ForegroundColor $TreeStyle.Color 
        }
        if ($LastWasPriority -and -not $IsPriority) { 
            Write-Host ($Indent + $TreeStyle.Vertical) -ForegroundColor $TreeStyle.Color
            $LastWasPriority = $false 
        }

        # --- GAŁĄŹ ---
        $BranchSymbol = if ($IsLast) { $TreeStyle.BranchLast } else { $TreeStyle.BranchMid }
        Write-Host ($Indent + $BranchSymbol) -NoNewline -ForegroundColor $TreeStyle.Color
        
        if ($Item.PSIsContainer) {
            # --- FOLDER ---
            Write-Host $DirStyle.Icon -NoNewline -ForegroundColor $DirStyle.IconColor
            Write-Host ($Item.Name + $DirStyle.Suffix) -ForegroundColor $DirStyle.TextColor
            
            # Obliczanie wcięcia dla dzieci
            $AddIndent = if ($IsLast) { $TreeStyle.IndentLast } else { $TreeStyle.IndentMid }
            $NextIndent = $Indent + $AddIndent
            
            Write-FileTree -CurrentPath $Item.FullName -Indent $NextIndent
        } else {
            # --- PLIK ---
            $FileStyle = Get-FileStyle -Item $Item
            Write-Host $FileStyle.Icon -NoNewline
            Write-Host $Item.Name -ForegroundColor $FileStyle.Color
            if ($IsPriority) { $LastWasPriority = $true }
        }
    }
}

# ==============================================================================
# 5. SILNIK LOGICZNY INTERPRETER WZORCÓW (REGEX, SCANNER, ZONES)
# ==============================================================================

function ConvertTo-Regex {
    param ([string]$Glob)
    $raw = $Glob
    $g = $Glob -replace '\\', '/'
    $isGeneric = ($g -eq "*" -or $g -eq "**/*")
    $onlyDir = $false
    if ($g.EndsWith("/")) { $onlyDir = $true; $g = $g.TrimEnd('/') }
    $g = [System.Text.RegularExpressions.Regex]::Escape($g)
    $g = $g -replace "\\\*\\\*", ".*"
    $g = $g -replace "\\\*", "[^/]*"
    $g = $g -replace "\\\?", "[^/]"
    $g = $g -replace "\[!", "[^"
    if ($g.StartsWith("/")) { $g = "^" + $g.Substring(1) } 
    else { if ($g.StartsWith(".*")) { $g = "^" + $g } else { $g = "(?:^|/)" + $g } }
    if ($onlyDir) { $g = $g + "(?:/.*)?$" } else { $g = $g + "$" }
    return [PSCustomObject]@{ Regex = $g; OnlyDir = $onlyDir; Raw = $raw; RawClean = ($raw -replace '\\', '/'); IsGeneric = $isGeneric }
}

function Scan-Dir {
    param ([string]$Path, [bool]$ParentIsRestricted)

    try { $items = Get-ChildItem -Path $Path -Force -ErrorAction SilentlyContinue } catch { return }

    foreach ($item in $items) {
        if ($item.FullName.Length -ge $rootLen) { $relPath = $item.FullName.Substring($rootLen).Replace('\', '/') } 
        else { $relPath = $item.Name }

        # 1. Wykluczenie
        $isExcludedByRule = $false
        foreach ($rule in $excludeRules) {
            if ($rule.OnlyDir -and -not $item.PSIsContainer) { continue }
            if (($relPath -match $rule.Regex) -or ($item.PSIsContainer -and "$relPath/" -match $rule.Regex)) {
                $isExcludedByRule = $true; break
            }
        }

        # 2. Stan Strefy
        $inRestrictedZone = $ParentIsRestricted -or $isExcludedByRule
        $shouldInclude = $false

        # 3. Match Specyficzny
        foreach ($rule in $specificIncludes) {
            if ($rule.OnlyDir -and -not $item.PSIsContainer) { continue }
            if (($relPath -match $rule.Regex) -or ($item.PSIsContainer -and "$relPath/" -match $rule.Regex)) {
                $shouldInclude = $true; break
            }
        }

        # 4. Match Ogólny
        if (-not $shouldInclude -and -not $inRestrictedZone) {
            foreach ($rule in $genericIncludes) { $shouldInclude = $true; break }
        }

        # 5. Tunelowanie
        $isTunnel = $false
        if (-not $shouldInclude -and $inRestrictedZone -and $item.PSIsContainer) {
            foreach ($inc in $specificIncludes) {
                if ($inc.RawClean.StartsWith("$relPath/")) { $isTunnel = $true; break }
            }
            if ($isTunnel) { $shouldInclude = $true }
        }

        if ($shouldInclude) {
            $allowedPaths.Add($item.FullName) | Out-Null
            $parent = Split-Path $item.FullName -Parent
            while ($parent.Length -ge ($resolvedRoot.Length) -and -not $allowedPaths.Contains($parent)) {
                $allowedPaths.Add($parent) | Out-Null
                $parent = Split-Path $parent -Parent
            }
            if ($item.PSIsContainer) { Scan-Dir -Path $item.FullName -ParentIsRestricted $inRestrictedZone }
        }
    }
}

# ==============================================================================
# 6. FUNKCJE POMOCNICZE (SETUP & UI)
# ==============================================================================

function Initialize-ScanContext {
    param ($Path, $Patterns)

    # 1. Rozwiązywanie ścieżki
    try {
        $rootObj = Resolve-Path $Path -ErrorAction Stop
        $resolvedRoot = $rootObj.Path
        $rootLen = $resolvedRoot.Length + 1 
    } catch { 
        throw "Krytyczny błąd: Ścieżka '$Path' nie istnieje." 
    }

    # 2. Kompilacja reguł
    $specIncludes = @()
    $genIncludes  = @()
    $excRules     = @()

    foreach ($p in $Patterns) {
        if ([string]::IsNullOrWhiteSpace($p)) { continue }
        $cleanP = $p.Trim()
        
        if ($cleanP.StartsWith("!")) { 
            $excRules += (ConvertTo-Regex $cleanP.Substring(1)) 
        } else {
            $r = ConvertTo-Regex $cleanP
            if ($r.IsGeneric) { $genIncludes += $r } else { $specIncludes += $r }
        }
    }

    # Zwracamy czysty obiekt kontekstu
    return [PSCustomObject]@{
        RootPath         = $resolvedRoot
        RootLen          = $rootLen
        SpecificIncludes = $specIncludes
        GenericIncludes  = $genIncludes
        ExcludeRules     = $excRules
    }
}

function Show-Header {
    param ($Context, $Patterns, $Strategy, $FoldersFirst)
    
    Write-Host "Lokalizacja: $($Context.RootPath)" -ForegroundColor Green
    Write-Host "Wzorce:      $($Patterns -join ', ')" -ForegroundColor DarkGray
    Write-Host "Tryb:        $Strategy | FoldersFirst: $FoldersFirst" -ForegroundColor DarkGray
    Write-Host "------------------------------------------------" -ForegroundColor Yellow
}

# ==============================================================================
# 7. MAIN (GŁÓWNY PRZEPŁYW STEROWANIA)
# ==============================================================================

try {
    # KROK 1: Inicjalizacja środowiska
    $Context = Initialize-ScanContext -Path $pathLocation -Patterns $pathPatterns

    # Przypisanie do zmiennych skryptu (wymagane przez funkcje Scan-Dir i ConvertTo-Regex)
    # W PowerShell funkcje czytają zmienne z zakresu rodzica, więc musimy je tu "rozpakować"
    $resolvedRoot     = $Context.RootPath
    $rootLen          = $Context.RootLen
    $specificIncludes = $Context.SpecificIncludes
    $genericIncludes  = $Context.GenericIncludes
    $excludeRules     = $Context.ExcludeRules
    
    $allowedPaths = [System.Collections.Generic.HashSet[string]]::new([System.StringComparer]::OrdinalIgnoreCase)

    # KROK 2: Wyświetlenie nagłówka
    Show-Header -Context $Context -Patterns $pathPatterns -Strategy $sortStrategy -FoldersFirst $sortDirFirst

    # KROK 3: Skanowanie (Silnik Logiczny)
    Scan-Dir -Path $resolvedRoot -ParentIsRestricted $false

    # KROK 4: Walidacja wyników
    if ($allowedPaths.Count -eq 0) { 
        Write-Warning "Brak wyników pasujących do filtrów."
        exit 
    }

    # KROK 5: Rysowanie (Warstwa Prezentacji)
    Write-FileTree -CurrentPath $resolvedRoot
    Write-Host "------------------------------------------------`n" -ForegroundColor Yellow

} catch {
    Write-Error $_.Exception.Message
    exit 1
}

# ==============================================================================