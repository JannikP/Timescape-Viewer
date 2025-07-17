use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Signals {
    /// Version of this signals definition structure. Should be "1.0".
    #[serde(rename = "@version")]
    version: String,

    /// Id of the next signal if the user would add one more.
    /// So far of no use for the Timescape-Viewer, as it does not write altered
    /// TIA trace configurations.
    #[serde(rename = "@nextSignalId")]
    next_signal_id: i64,

    /// All defined signals in this TIA trace configuration.
    #[serde(rename = "RecordSignal")]
    signals: Vec<RecordSignal>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct RecordSignal {
    /// Internal ID of this signal. Usually a monotonic increasing number.
    #[serde(rename = "@signalId")]
    signal_id: i64,

    /// ?
    #[serde(rename = "@isSelected")]
    is_selected: bool,

    /// Highest visible y value.
    #[serde(rename = "@maxVisibleY")]
    max_visible_y: f64,

    /// Lowest visible y value.
    #[serde(rename = "@minVisibleY")]
    min_visible_y: f64,

    /// Display format such as "Float"
    #[serde(rename = "@displayFormat")]
    display_format: String,

    /// ?
    #[serde(rename = "ConfigurationSignal")]
    configuration: ConfigurationSignal,

    /// Color of this trace in TIA Portal's own trace viewer.
    /// Yes, they use British english.
    #[serde(rename = "Visualisation")]
    visualization: Visualization,

    /// Physical unit of the recoded signal. E. g. "mm/s".
    #[serde(rename = "Unit")]
    unit: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct ConfigurationSignal {

    /// Internal ID of this signal. Usually a monotonic increasing number.
    #[serde(rename = "@signalId")]
    signal_id: i32,

    /// Full path of the signal's variable in the PLC program.
    /// E. g. "My Kinematic".StatusPath.Acceleration
    #[serde(rename = "Name")]
    name: String,

    /// Optional comment of the signal's variable in the PLC program.
    /// If there is no comment, this is parsed as empty string.
    #[serde(rename = "Comment")]
    comment: String,

    /// Number of this signal in the trace configuration.
    /// Monotonic increasing sequence starting from one.
    #[serde(rename = "Label")]
    label: i64,

    /// Visualization and address information.
    /// Yes, they use British english and mix visualization and address information.
    #[serde(rename = "Visualisation")]
    visualization: Visualization,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Visualization {
    /// Display color of the trace in TIA Portal's own trace viewer.
    /// Most likely RGBA values. The individual components are in decimal
    /// notation between 0 and 255, separated by a comma.
    /// E. g. "255,0,0,255".
    #[serde(rename = "@displayColor")]
    display_color: String,

    /// The visualization structure inside of the configuration signal
    /// has additional address information, while the visualization
    /// structure directly under the record signal does not.
    #[serde(rename = "AddressInformation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    address_information: Option<AddressInformation>,
}

/// Information about the recorded variable on the PLC.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct AddressInformation {
    /// Data type on the PLC, such as "LReal".
    /// TODO: This should be an `enum` once we figured out all possible options.
    #[serde(rename = "@dataType")]
    data_type: String,

    /// Data type in C#, such as "Double".
    /// TODO: This should be an `enum` once we figured out all possible options.
    #[serde(rename = "@dataTypeCode")]
    data_type_code: String,

    /// Size of the data type in bits. For example 64 for LReal/Double.
    #[serde(rename = "@bitSize")]
    bit_size: i32,

    /// ?
    /// Zero for LReal.
    #[serde(rename = "@bitOffset")]
    bit_offset: i32,

    /// How to display the value to the user.
    /// Observed options are "Float".
    #[serde(rename = "@displayFormatPreset")]
    display_format_preset: String,

    /// Icon of this signal inside TIA's trace viewer.
    /// Observed options are "DB".
    /// No use for the Timescape-Viewer.
    #[serde(rename = "@iconType")]
    icon_type: String,
}

#[cfg(test)]
mod tests {
    use serde_xml_rs::from_str;

    use super::{AddressInformation, ConfigurationSignal, RecordSignal, Signals, Visualization};

    #[test]
    fn parse_signals() {
        // Arrange
        let src = r#"<?xml version="1.0" encoding="utf-8"?>
        <Signals xmlns:xsd="http://www.w3.org/2001/XMLSchema"
            xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" version="1.0" nextSignalId="94">
            <RecordSignal signalId="0" isSelected="true" maxVisibleY="1500" minVisibleY="0" displayFormat="Float">
                <ConfigurationSignal signalId="0">
                    <Name>"My Kinematic".StatusPath.Velocity</Name>
                    <Comment />
                    <Label>0</Label>
                    <Visualisation displayColor="255,255,37,87">
                        <AddressInformation dataType="LReal" dataTypeCode="Double" bitSize="64" bitOffset="0" displayFormatPreset="Float" iconType="DB" />
                    </Visualisation>
                </ConfigurationSignal>
                <Visualisation displayColor="255,255,37,87" />
                <Unit>mm/s</Unit>
            </RecordSignal>
            <RecordSignal signalId="1" isSelected="true" maxVisibleY="25000" minVisibleY="-25000" displayFormat="Float">
                <ConfigurationSignal signalId="1">
                    <Name>"My Kinematic".StatusPath.Acceleration</Name>
                    <Comment />
                    <Label>1</Label>
                    <Visualisation displayColor="255,0,0,255">
                        <AddressInformation dataType="LReal" dataTypeCode="Double" bitSize="64" bitOffset="0" displayFormatPreset="Float" iconType="DB" />
                    </Visualisation>
                </ConfigurationSignal>
                <Visualisation displayColor="255,0,0,255" />
                <Unit>mm/s²</Unit>
            </RecordSignal>
        </Signals>
        "#;
        // Act
        let result: Signals = from_str(src).expect("Failed to parse XML snippet.");
        // Assert
        assert_eq!(result.version, "1.0", "Version mismatch.");
        assert_eq!(result.next_signal_id, 94, "Next signal ID mismatch.");
        assert_eq!(result.signals.len(), 2, "There should be two signals.");
    }

    #[test]
    fn parse_record_signal() {
        // Arrange
        let src = r#"
        <RecordSignal signalId="0" isSelected="true" maxVisibleY="1500" minVisibleY="0" displayFormat="Float">
            <ConfigurationSignal signalId="0">
                <Name>"My Kinematic".StatusPath.Velocity</Name>
                <Comment />
                <Label>0</Label>
                <Visualisation displayColor="255,255,37,87">
                    <AddressInformation dataType="LReal" dataTypeCode="Double" bitSize="64" bitOffset="0" displayFormatPreset="Float" iconType="DB" />
                </Visualisation>
            </ConfigurationSignal>
            <Visualisation displayColor="255,255,37,87" />
            <Unit>mm/s</Unit>
        </RecordSignal>
        "#;
        // Act
        let result: RecordSignal = from_str(src).expect("Failed to parse XML snippet.");
        // Assert
        assert_eq!(result.signal_id, 0, "Signal ID mismatch.");
        assert_eq!(result.is_selected, true, "Should be selected.");
        assert_eq!(result.max_visible_y, 1500.0, "Max visible Y mismatch.");
        assert_eq!(result.min_visible_y, 0.0, "Min visible Y mismatch.");
        assert_eq!(result.display_format, "Float", "Display format mismatch.");
        // Configuration signal has its own unit test. Not checked here.
        // Visualization has its own unit test. Not check here.
        assert_eq!(result.unit, "mm/s", "Unit mismatch.");
    }

    #[test]
    fn parse_configuration_signal() {
        // Arrange
        let src = r#"
        <ConfigurationSignal signalId="0">
            <Name>"My Kinematic".StatusPath.Velocity</Name>
            <Comment />
            <Label>0</Label>
            <Visualisation displayColor="255,255,37,87">
                <AddressInformation dataType="LReal" dataTypeCode="Double" bitSize="64" bitOffset="0" displayFormatPreset="Float" iconType="DB" />
            </Visualisation>
        </ConfigurationSignal>
        "#;
        // Act
        let result: ConfigurationSignal = from_str(src).expect("Failed to parse XML snippet.");
        // Assert
        assert_eq!(result.signal_id, 0, "Signal id mismatch.");
        assert_eq!(result.name, "\"My Kinematic\".StatusPath.Velocity", "Name mismatch.");
        assert_eq!(result.comment, "", "Comment mismatch.");
        assert_eq!(result.label, 0, "Label mismatch.");
        // Visualization has its own unit test. Not checked here.
    }

    #[test]
    fn parse_visualization_without_address_information() {
        // Arrange
        let src = r#"<Visualisation displayColor="255,255,37,87" />"#;
        // Act
        let result: Visualization = from_str(src).expect("Failed to parse XML snippet.");
        // Assert
        assert_eq!(result.display_color, "255,255,37,87", "Display color mismatch.");
        assert_eq!(result.address_information, None, "Address information should be none.");
    }

    #[test]
    fn parse_visualization_with_address_information() {
        // Arrange
        let src = r#"
        <Visualisation displayColor="255,255,37,87">
            <AddressInformation dataType="LReal" dataTypeCode="Double" bitSize="64" bitOffset="0" displayFormatPreset="Float" iconType="DB" />
        </Visualisation>
        "#;
        // Act
        let result: Visualization = from_str(src).expect("Failed to parse XML snippet.");
        // Assert
        assert_eq!(result.display_color, "255,255,37,87", "Display color mismatch.");
        assert!(matches!(result.address_information, Some(_)), "This snippet does contain address information.");
    }

    #[test]
    fn parse_address_information() {
        // Arrange
        let src = r#"
        <AddressInformation dataType="LReal" dataTypeCode="Double" bitSize="64" bitOffset="0" displayFormatPreset="Float" iconType="DB" />
        "#;
        // Act
        let result: AddressInformation = from_str(src).expect("Failed to parse XML snippet.");
        // Assert
        assert_eq!(result.data_type, "LReal", "Data type mismatch.");
        assert_eq!(result.data_type_code, "Double", "Data type code mismatch.");
        assert_eq!(result.bit_size, 64, "Bit size mismatch.");
        assert_eq!(result.bit_offset, 0, "Bit offset mismatch.");
        assert_eq!(result.display_format_preset, "Float", "Display format preset mismatch.");
        assert_eq!(result.icon_type, "DB", "Icon type mismatch.");
    }
}
