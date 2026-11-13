# Astro licensing: distributions and MIT permission for 7mlabs material

[Tiếng Việt](license.md) · [Permission scope](../LICENSE-SCOPE.md) · [Scope inventory](../LICENSE-SCOPE.json)

## At a glance

**The public combined native distribution uses `AGPL-3.0-only`. Identified original contributions owned by 7mlabs additionally have a separate MIT permission.** That permission does not license the entire SDK, Swiss Ephemeris or bundled data under MIT.

- **Covered 7mlabs material:** use the [standard MIT permission](../LICENSE-7MLABS-MIT) within its [precise scope](../LICENSE-SCOPE.md). When using that permission, there is no independent 7mlabs-only AGPL restriction on the covered contributions.
- **Swiss Ephemeris:** retains its upstream AGPL / Swiss Ephemeris Professional options. The 7mlabs MIT grant does not supply Swiss rights.
- **A combined work using Swiss:** needs appropriate permission for every component. MIT permission for one component does not exempt the entire combined binary from AGPL.

**SDK 1.1.3** is the first registry release to bundle `LICENSE-7MLABS-MIT`, `LICENSE-SCOPE.md`, `LICENSE-SCOPE.json` and the updated `NOTICE` alongside corresponding source. Whole-package metadata remains `AGPL-3.0-only`. See [the 1.1.3 release record](release-1.1.3.md) for the distinction between required checks and finalized evidence.

The already published npm/PyPI/NuGet **1.1.2 archives retain their original bytes, licensing contents and release evidence**. The permission documented here is an additional grant for contributions identified in a source revision carrying the grant files. It does not say that the published 1.1.2 archives have been repackaged or become MIT distributions.

## License map

| Material | Applicable permission |
| --- | --- |
| Original 7mlabs-owned code identified in `LICENSE-SCOPE.json` | Additional MIT permission in `LICENSE-7MLABS-MIT`, with precise scope in `LICENSE-SCOPE.md` |
| Original 7mlabs additions/modifications to Swiss source identified by the inventoried diff | MIT for those 7mlabs contributions only, not for the whole mixed Swiss file |
| Vendored Swiss and embedded Chiron data | Swiss terms; the public combined native distribution chooses AGPL |
| Complete combined native binary in the public distribution | `AGPL-3.0-only`; no unconditional MIT option for the complete binary |
| Independent `examples/frontend/ui` renderer | Its [separate MIT license](../examples/frontend/ui/LICENSE) |
| Rust dependencies, timezone data, fonts and other third-party assets | Their own retained notices and licenses; see [NOTICE](../NOTICE) |
| Names, trademarks and brand assets | Not automatically licensed by the MIT code grant; see the applicable scope and notices |

The inventory identifies revisions, files and hashes to distinguish covered contributions from excluded material. In modified Swiss files, original upstream lines, deleted lines and unchanged context shown in a diff do not become MIT-licensed 7mlabs code. Swiss terms governing the complete file and its modification/distribution remain applicable.

## Two routes for using Swiss

### The AGPL route

AGPL permits both free-of-charge and commercial use. Distribution of a covered work requires retained notices and corresponding source under sections 4–6. Section 13 requires an offer of corresponding source to users interacting remotely with a modified program. Assess obligations for applications combined with the engine under the applicable scope of AGPL.

Corresponding source must match the version actually distributed or operated, including relevant modifications and necessary build scripts. Merely linking to the upstream repository does not by itself satisfy obligations for your modified version.

### The Swiss Professional route

A holder of a valid Professional license can rely on their Swiss permission **within the scope of their actual agreement**, alongside the MIT permission for the identified 7mlabs material. 7mlabs imposes no independent AGPL-only restriction on the contributions covered by that MIT permission. This does not determine whether a particular product, service, data set, modification or binary is covered by the holder's Swiss agreement.

**We have not confirmed that the existing prebuilt combined native binaries can automatically be used under Professional.** They embed Swiss, Chiron data and modified Swiss source. Clarify with Swiss how the agreement applies to that combination before advertising such a guarantee. Rebuilding from source also does not replace the need for sufficient Swiss and other third-party rights.

Professional-user checklist:

1. Verify that your signed Swiss agreement is valid and covers the right entity, product/service and intended use.
2. Identify the source revision, data, Swiss changes and binary you intend to use; check `LICENSE-SCOPE.md` and provenance.
3. Clarify rights for the specific combination with Swiss where your agreement is unclear, particularly modified source, bindings/ports or third-party API services.
4. Retain MIT for the 7mlabs material, Swiss notices and dependency licenses in distributed products as required by their respective terms.

The public Swiss agreement checked on **12 November 2026** is the September 2026 edition updated on 9 November 2026. Section 5 concerns source distribution and modifications; section 13 concerns bindings/ports and the two licensing routes. Each recipient's signed agreement determines their actual Professional rights; do not assume older agreements have the same terms as the current form.

## Packaging and compatibility

`LICENSE` and the combined native package metadata continue to state `AGPL-3.0-only`. Do not label the whole package `MIT` or `AGPL-3.0-only OR MIT`. From 1.1.3, the additional MIT/scope files and updated `NOTICE` accompany installation packages and source archives to identify separate permission for the covered 7mlabs contributions. Retain the full AGPL text, `NOTICE`, Swiss notices, provenance and dependency licenses alongside packages/source archives newly prepared under this policy.

The additional permission changes no functions, algorithms, JSON, ABI or calling conventions and requires no application code changes. The SDK has no Professional-key entry or verification mechanism. A successful calculation does not establish licensing entitlement.

Historical releases and older source snapshots retain their contents at release time. The repository does not import legacy application source/history; [legacy notes](legacy-source.md) describe historical context. Do not extend this MIT permission to future contributions or material outside the identified scope.

## Governing files and sources

- [7mlabs permission scope](../LICENSE-SCOPE.md), [standard MIT text](../LICENSE-7MLABS-MIT), [inventory](../LICENSE-SCOPE.json), [full AGPL](../LICENSE) and [NOTICE](../NOTICE).
- [Swiss license at the pinned commit](https://github.com/aloistr/swisseph/blob/aacf962d19d79f8bc921dbcdaacf306d85be1917/LICENSE) and [provider provenance](../neutral-engine/crates/astro-provider-swiss/PROVENANCE.md).
- [Swiss licensing guide](https://www.astro.com/swisseph/swephinfo_e.htm) and [current Professional contract form](https://www.astro.com/swisseph/secont_e_2609.pdf).
- [GNU AGPL v3, especially sections 5 and 7](https://www.gnu.org/licenses/agpl-3.0.html), [GNU licensing compatibility and relicensing guidance](https://www.gnu.org/licenses/license-compatibility.en.html).

This guide explains how to read the grant files. The actual license texts, each copyright holder's rights and applicable agreements determine the exact permissions; this guide does not expand Swiss rights.
