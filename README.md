# [🦀☯️🦕 lekcja 002](https://github.com/mod-by-cis/learn-rust-step002-database)

|  | ▶️ PS | 🦕 deno | 🦀 rust |
| --- | ----- | ------- | ------- |
| **v**: | $7.5.4$ | $2.6.6$ | $1.93.0 |

Projek zasadzniczo składa się z trzech niezależnych - lub częściowo niezależnych projektów:

* 🦀 `./src-rust` - <b>*CLI totalnie niezależny, serce programu - konsolowe*</b>
* ☯️ `./src-tauri` - <b>*GUI na windows i android*</b>
* 🦕 `./src-deno` - <b>*interfejs graficzny*</b>

Reszte stanowią pliki konfiguracyjne, lub narzędziowe

- - -

## CMD (KOMENDY)

### 🦀 RUST

* Lista obszarów-roboczych

> ``` PowerShell
> cargo metadata --no-deps --format-version 1 | ConvertFrom-Json | Select-Object -ExpandProperty workspace_members
> ```
> 
> inna opcja
> 
> 
> ``` PowerShell
> cargo tree --depth 1
> ```

dd