use rocksdb::{DB, WriteOptions};
use std::sync::Arc;
use bincode::{encode_to_vec, config::standard};
use log::{debug, error};
use crate::entities::mqtt_entity::MqttEntity;
use crate::repositories::repository_error::MqttRepositoryError;

pub struct SoftDeleteMqttRepository {
    db: Arc<DB>,
}

impl SoftDeleteMqttRepository {
    pub fn new(db: Arc<DB>) -> Self {
        SoftDeleteMqttRepository { db }
    }

    pub fn soft_delete(&self, mqtt: MqttEntity) -> Result<(), MqttRepositoryError> {
        // Build RocksDB key
        let key: String = format!("mqtt:{}", mqtt.username);

        // Set is_deleted to true
        let mut updated_mqtt = mqtt;
        updated_mqtt.is_deleted = true;

        // Encode the updated entity
        debug!("[Repository | SoftDeleteMQTT] Encoding updated user MQTT data for '{}'.", updated_mqtt.username);
        let encoded = match encode_to_vec(&updated_mqtt, standard()) {
            Ok(data) => data,
            Err(e) => {
                error!("[Repository | SoftDeleteMQTT] Failed to encode user MQTT data for {}: {e}", updated_mqtt.username);
                debug!("[Repository | SoftDeleteMQTT] Encode error for user MQTT '{}': {:#?}", updated_mqtt.username, e);
                return Err(MqttRepositoryError::Encode(e));
            }
        };

        // Write updated entity to DB
        let mut write_opts = WriteOptions::default();
        write_opts.disable_wal(false);

        debug!("[Repository | SoftDeleteMQTT] Writing updated user MQTT '{}' to database.", updated_mqtt.username);
        match self.db.put_opt(key.as_bytes(), &encoded, &write_opts) {
            Ok(_) => {
                debug!("[Repository | SoftDeleteMQTT] Successfully soft deleted user MQTT '{}'.", updated_mqtt.username);
                Ok(())
            }
            Err(e) => {
                error!("[Repository | SoftDeleteMQTT] Database write error for user MQTT {}: {e}", updated_mqtt.username);
                debug!("[Repository | SoftDeleteMQTT] Database write error for user MQTT '{}': {:#?}", updated_mqtt.username, e);
                Err(MqttRepositoryError::Database(e))
            }
        }
    }
}
