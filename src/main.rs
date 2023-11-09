// ***** Пример библиотеки "Умный дом" со статическим содержимым

trait Device {
    fn get_description(&self) -> String;
}

struct SmartHouse {
    rooms: Vec<Room>,
}

struct Room {
    name: String,
    devices: Vec<Box<dyn Device>>,
}

impl SmartHouse {
    fn new(rooms: Vec<Room>) -> Self {
        SmartHouse { rooms }
    }

    fn get_rooms(&self) -> Vec<String> {
        self.rooms.iter().map(|room| room.name.clone()).collect()
    }

    fn devices(&self, room: &str) -> Vec<String> {
        if let Some(room_name) = self.rooms.iter().find(|r| r.name == room) {
            room_name
                .devices
                .iter()
                .map(|device| device.get_description())
                .collect()
        } else {
            Vec::new() // Возвращаем пустой вектор, если комната не найдена
        }
    }

    fn create_report<T: DeviceInfoProvider>(&self, provider: &T) -> String {
        let mut report = String::new();

        // Используем get_rooms для получения списка комнат
        for room_name in self.get_rooms() {
            // Добавляем название комнаты в отчет
            report.push_str(&format!("Room '{}':\n", room_name));

            // Получаем список устройств для данной комнаты
            for device_name in self.devices(&room_name) {
                // Используем provider для получения информации об устройстве
                match provider.get_device_info(&room_name, &device_name) {
                    Ok(info) => {
                        // Добавляем информацию об устройстве в отчет
                        report.push_str(&format!("  {}\n", info));
                    }
                    Err(e) => {
                        // Если устройство не найдено, добавляем сообщение об ошибке в отчет
                        report.push_str(&format!("  Error: {}\n", e));
                    }
                }
            }

            // Добавляем пустую строку после каждой комнаты для лучшей читаемости
            report.push('\n');
        }

        report
    }
}

trait DeviceInfoProvider {
    fn get_device_info(&self, room_name: &str, device_name: &str) -> Result<String, String>;
}

#[derive(Clone)]
struct SmartSocket {
    name: String,
    power_consumption: f64,
    is_on: bool,
}

impl Device for SmartSocket {
    fn get_description(&self) -> String {
        self.name.to_string()
    }
}

#[derive(Clone)]
struct SmartThermometer {
    name: String,
    temperature: f64,
}

impl Device for SmartThermometer {
    fn get_description(&self) -> String {
        // format!("{}", self.name)
        self.name.to_string()
    }
}

struct OwningDeviceInfoProvider {
    socket: SmartSocket,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn get_device_info(&self, room_name: &str, device_name: &str) -> Result<String, String> {
        if self.socket.name == device_name {
            Ok(format!(
                "The device '{}' in the room '{}' is {} with a power consumption of {} watts.",
                self.socket.name,
                room_name,
                if self.socket.is_on { "On" } else { "Off" },
                self.socket.power_consumption
            ))
        } else {
            // Err(format!("Device '{}' not found in room '{}'.", device_name, room_name))
            Err(("").to_string())
        }
    }
}

struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a SmartSocket,
    thermo: &'b SmartThermometer,
}

impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn get_device_info(&self, _room_name: &str, device_name: &str) -> Result<String, String> {
        if self.socket.name == device_name {
            Ok(format!(
                "Socket '{}' is currently {} with a power consumption of {} watts.",
                self.socket.name,
                if self.socket.is_on { "on" } else { "off" },
                self.socket.power_consumption
            ))
        } else if self.thermo.name == device_name {
            Ok(format!(
                "Thermometer '{}' reads a temperature of {} degrees Celsius.",
                self.thermo.name, self.thermo.temperature
            ))
        } else {
            // Err(format!("Device '{}' not found in room '{}'.", device_name, _room_name))
            Err("".to_string())
        }
    }
}

fn main() {
    let socket1 = SmartSocket {
        name: "Living room socket".to_string(),
        power_consumption: 150.0,
        is_on: true,
    };

    let socket2 = SmartSocket {
        name: "Bedroom socket".to_string(),
        power_consumption: 250.0,
        is_on: true,
    };

    let thermo = SmartThermometer {
        name: "Bedroom Thermometer".to_string(),
        temperature: 23.4,
    };

    let living_room_devices: Vec<Box<dyn Device>> = vec![Box::new(socket1.clone())];
    let bedroom_devices: Vec<Box<dyn Device>> =
        vec![Box::new(socket2.clone()), Box::new(thermo.clone())];

    let living_room = Room {
        name: "Living room".to_string(),
        devices: living_room_devices,
    };
    let bedroom = Room {
        name: "Bedroom".to_string(),
        devices: bedroom_devices,
    };

    let house = SmartHouse::new(vec![living_room, bedroom]);

    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };

    let report1 = house.create_report(&info_provider_1);

    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };
    let report2 = house.create_report(&info_provider_2);

    // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}
