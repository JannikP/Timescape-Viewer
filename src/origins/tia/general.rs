//! This module contains data structures to decode the `General` section of a
//! TIA trace.
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct General {
    /// Version of this general information structure. Should be "1.0".
    #[serde(rename = "@version")]
    pub version: String,

    /// Creation time and date of this trace in ISO 8601 format.
    /// For example: "2025-07-15T06:14:51.0023134Z"
    #[serde(rename = "CreationTime")]
    pub creation_time: Timestamp,

    /// modification time and date of this trace in ISO 8601 format.
    /// For example: "2025-07-15T06:14:49.0637924Z"
    #[serde(rename = "ModificationTime")]
    pub modification_time: Timestamp,

    /// Type and firmware information of the recording PLC.
    #[serde(rename = "Device")]
    pub device: Device,

    /// Unknown. Something like "Siemens.TechTrace.Device.Plc.S71500.TraceDeviceStrategy"
    #[serde(rename = "StrategyName")]
    pub strategy_name: String,

    /// Title of this trace configuration.
    #[serde(rename = "ConfigurationName")]
    pub configuration_name: String,

    /// The configuration's author's name, if given.
    #[serde(rename = "ConfigurationAuthor")]
    pub configuration_author: Option<String>,

    /// UUID of this configuration such as "93cb2eb3-0f8f-45f7-8bf1-28b2a5a082f8".
    /// The Timescape-Viewer doesn't care about it, so we don't parse it into a `uuid::Uuid`.
    #[serde(rename = "ConfigurationId")]
    pub configuration_id: String,

    /// Unknown. Something like "Siemens.TechTrace.Editor.20180FD9-9F1D-49C6-902A-C1E033689203".
    #[serde(rename = "TraceClientId")]
    pub trace_client_id: String,

    /// Exact date and time when a triggered trace was triggered.
    /// For example: "2025-07-15T06:14:49.0637924Z"
    #[serde(rename = "TriggerTime")]
    pub trigger_time: Option<Timestamp>,

    /// Unknown. Usually "RecordingCompleted" when we get it.
    #[serde(rename = "JobState")]
    pub job_state: String,

    /// Title of this trace recording.
    #[serde(rename = "RecordName")]
    pub record_name: String,

    /// The configuration's author's name, if given.
    #[serde(rename = "RecordAuthor")]
    pub record_author: Option<String>,

    /// Timestamp of when this recording was started.
    /// This might be hours or even days before the trigger happened.
    /// For example: "2025-07-14T15:54:34.4926704Z"
    #[serde(rename = "ActivationTime")]
    pub activation_time: Timestamp,

    /// Timestamp of the very first sample in this recording.
    /// For example: "2025-07-14T21:10:59.0926704Z"
    #[serde(rename = "FirstSampleTime")]
    pub first_sample_time: Timestamp,

    /// Timestamp of the very last recording.
    /// Should be equal to `first_sample_time` +
    /// For example: "2025-07-14T21:11:01.0886704Z"
    #[serde(rename = "LastSampleTime")]
    pub last_sample_time: Timestamp,

    /// Duration of the recording. Time between `first_sample_time` and `last_sample_time`.
    /// TODO: Unknown unit.
    #[serde(rename = "RecordingDuration")]
    pub recording_duration: i64,

    /// Number of samples before the trigger event. Might be zero without pre-trigger.
    #[serde(rename = "SamplesBeforeTrigger")]
    pub samples_before_trigger: i64,

    /// Number of samples after the trigger event.
    #[serde(rename = "SamplesAfterTrigger")]
    pub samples_after_trigger: i64,

    /// Number of actually recorded samples.
    #[serde(rename = "ActualSamples")]
    pub actual_samples: i64,

    /// Maximum number of samples that could be recorded.
    /// Maybe different from `actual_samples` if the user stops the recording
    /// before `maximum_samples` where collected? ¯\\_(ツ)_/¯
    #[serde(rename = "MaximumSamples")]
    pub maximum_samples: i64,

    /// Number of microseconds between two samples.
    #[serde(rename = "CycleTime")]
    pub cycle_time: i64,

    /// Number of nanoseconds between two samples.
    #[serde(rename = "CycleTimeInNs")]
    pub cycle_time_in_ns: i64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Device {
    /// Device category such as "PLC S71500"
    #[serde(rename = "@category")]
    pub category: String,

    /// Device type such as "CPU 1518TF-4 PN/DP"
    #[serde(rename = "@type")]
    pub device_type: String,

    /// Siemens order number (MLFB) such as "6ES7 518-4UP00-0AB0"
    #[serde(rename = "@orderNumber")]
    pub order_number: String,

    /// The PLC's firmware version such as "V3.1"
    #[serde(rename = "@FirmwareVersion")]
    pub firmware_version: String,
}

#[cfg(test)]
mod tests {
    use jiff::civil::date;
    use serde_xml_rs::from_str;

    use super::{Device, General};

    #[test]
    fn parse_general_information() {
        // Arrange
        let src = r#"
        <General version="1.0">
            <CreationTime>2025-07-15T06:14:51.0023134Z</CreationTime>
            <ModificationTime>2025-07-15T06:14:49.0637924Z</ModificationTime>
            <Device category="PLC S71500" type="CPU 1518TF-4 PN/DP" orderNumber="6ES7 518-4UP00-0AB0" FirmwareVersion="V3.1" />
            <StrategyName>Siemens.TechTrace.Device.Plc.S71500.TraceDeviceStrategy</StrategyName>
            <ConfigurationName>Example Trace Configuration</ConfigurationName>
            <ConfigurationAuthor />
            <ConfigurationId>93cb2eb3-0f8f-45f7-8bf1-28b2a5a082f8</ConfigurationId>
            <TraceClientId>Siemens.TechTrace.Editor.20180FD9-9F1D-49C6-902A-C1E033689203</TraceClientId>
            <TriggerTime>2025-07-14T21:11:00.0926704Z</TriggerTime>
            <JobState>RecordingCompleted</JobState>
            <RecordName>Example Trace Configuration</RecordName>
            <RecordAuthor />
            <ActivationTime>2025-07-14T15:54:34.4926704Z</ActivationTime>
            <FirstSampleTime>2025-07-14T21:10:59.0926704Z</FirstSampleTime>
            <LastSampleTime>2025-07-14T21:11:01.0886704Z</LastSampleTime>
            <RecordingDuration>1996000000</RecordingDuration>
            <SamplesBeforeTrigger>250</SamplesBeforeTrigger>
            <SamplesAfterTrigger>250</SamplesAfterTrigger>
            <ActualSamples>500</ActualSamples>
            <MaximumSamples>500</MaximumSamples>
            <CycleTime>4000</CycleTime>
            <CycleTimeInNs>4000000</CycleTimeInNs>
        </General>
        "#;
        // Act
        let result: General = from_str(src).expect("Failed to parse XML snippet.");
        // Assert
        assert_eq!(result.version, "1.0", "Version mismatch.");
        assert_eq!(
            result.creation_time,
            "2025-07-15T06:14:51.0023134Z".parse().unwrap(),
            "CreationTime mismatch."
        );
        assert_eq!(
            result.modification_time,
            date(2025, 7, 15)
                .at(6, 14, 49, 63792400)
                .in_tz("UTC")
                .unwrap()
                .timestamp(),
            "ModificationTime mismatch."
        );
        // TODO: Many more field to check...
        assert_eq!(result.cycle_time_in_ns, 4000000);
    }

    #[test]
    fn parse_device() {
        let src = r#"<Device category="PLC S71500" type="CPU 1518TF-4 PN/DP" orderNumber="6ES7 518-4UP00-0AB0" FirmwareVersion="V3.1" />"#;
        let result: Device = from_str(src).expect("Failed to parse XML snippet.");
        assert_eq!(result.category, "PLC S71500");
        assert_eq!(result.device_type, "CPU 1518TF-4 PN/DP");
        assert_eq!(result.order_number, "6ES7 518-4UP00-0AB0");
        assert_eq!(result.firmware_version, "V3.1");
    }
}
