# Planned and Supported Data Sources

Timescape-Viewer is planned to read a wide spectrum of timeseries data sources
including a number of common file formats as well as realtime sources.

As of now no sources are fully implemented and the following list is for
brainstorming and idea collection.

| Format           | Type | Specification               | Crate                                     |
|------------------|------|-----------------------------|-------------------------------------------|
| Apache Arrow     | IPC  | [Arrow Homepage](https://arrow.apache.org/) | [arrow](https://crates.io/crates/arrow) |
| CSV files        | File | No standard, content varies | [csv](https://crates.io/crates/csv)       |
| TIA Portal Trace | File | Proprietary                 | None                                      |
| MATLAB files     | File | TODO                        | TODO                                      |
| MDF5             | File | TODO                        | TODO                                      |
| Audio formats    | File | TODO                        | TODO                                      |
| Timescale DB     | DB   | TODO                        | TODO                                      |
| Probe.rs & defmt | Live | TODO                        | TODO                                      |
| SIMATIC Trace    | Live | Proprietary                 | TODO                                      |
| Logic Analyzer   | Live | TODO                        | TODO                                      |
| Hantek Oszi.     | Live | TODO                        | TODO                                      |
| Multimeter USB   | Live | TODO                        | TODO                                      |
