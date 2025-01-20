# Universal Robots Ethernet Communication Protocol

[Universal Robots Website](https://www.universal-robots.com/ "Universal Robots")

[Universal Robots RTDE How To Guide](https://www.universal-robots.com/articles/ur/interface-communication/real-time-data-exchange-rtde-guide/)

## Quickstart

Test the interface with a simulated Universal Robot from dockerhub "universalrobots/ursim_e-series"

Set this up by installing docker and running `docker compose -f "./examples/simulator/docker-compose.yaml" up --build`.

Then run the example with `cargo run --example demo`.

Run tests with `cargo test -- --test-threads=1` to stop it trying to paralellize the tests (which rely on the same hardware).

## Introduction

The Universal Robot can be controlled at two levels:

* The PolyScope or the Graphical User Interface Level
* Script Level

At the Script Level, the URScript is the programming language that controls the robot.
The URScript includes variables, types, and the flow control statements. There are also
built-in variables and functions that monitor and control I/O and robot movements.

## Real-Time Data Exchange (RTDE)

The RTDE synchronize external executables with the UR controller, for instance URCaps, without breaking any real-time properties.

The Real-Time Data Exchange (RTDE) interface provides a way to synchronize external applications with the UR controller over a standard TCP/IP connection, without breaking any real-time properties of the UR controller. This functionality is among others useful for interacting with drivers (e.g. Ethernet/IP), manipulating robot I/O and plotting robot status (e.g. robot trajectories). The RTDE interface is by default available when the UR controller is running. The synchronization is configurable and can for example involve the following data:

* Output: robot-, joint-, tool- and safety status, analog and digital I/O's and general purpose output registers
* Input: digital and analog outputs and general purpose input registers
  
The RTDE functionality is split in two stages: a setup procedure and a synchronization loop.

On connection to the RTDE interface, the client is responsible for setting up the variables to be synchronized. Any combination of input and output registers that the client needs to write and read, respectively, can be specified. To achieve this the client sends a setup list of named input and output fields that should be contained in the actual data synchronization packages. The definition of a synchronization data package format is referred to as a recipe. There is a maximum limit of 2048 bytes to represent the list of inputs/outputs field names a client would like to subscribe to. In return the RTDE interface replies with a list of the variable types or specifies that a specific variable has not been found. Each input recipe that has been successfully configured will get a unique recipe id. The list of supported field names and their associated data types can be found below. When the setup is complete the data synchronization can be started and paused.

When the synchronization loop is started, the RTDE interface sends the client the requested data in the same order it was requested by the client. Furthermore the client is expected to send updated inputs to the RTDE interface on a change of value. The data synchronization uses serialized data.

All packages share the same general structure with a header and a payload if applicable. The packages used for the setup procedure generate a return message. The synchronization loop packages do not. Both client and server may at any time send a text message, whereby the warning level specifies the severity of the problem. The RTDE is available on port number 30004.

### Key Features

* **Real-time synchronization:** The RTDE generally generates output messages on 125 Hz. However, the real-time loop in the controller has a higher priority than the RTDE interface. Therefore, if the controller lacks computational resources it will skip a number of output packages. The skipped packages will not be sent later, the controller will always send the most recent data. Input packages will always be processed in the control cycle they are received, so the load for the controller will vary depending on the number of received packages.
* **Input messages:** The updating of variables in the controller can be divided into multiple messages. One can have one message to update everything or a message per variable or any division in between. There is no need for a constant update rate; inputs keep their last received value. Note: Only one RTDE client is allowed to control a specific variable at any time.
Runtime environment: An RTDE client may run on the UR Control Box PC or on any external PC. The advantage of running the RTDE client on the Control Box is no network latency. However, the RTDE client and UR controller will compete for computational resources. Please make sure that the RTDE client runs with standard operating system priority. Computationally intensive processes, e.g. image processing, should be run on an external PC.
* **Protocol changes:** The RTDE protocol might be updated at any time by UR. To guarantee maximum compatibility for your RTDE client, RTDE clients can request the RTDE interface to speak a specific protocol version. Protocol additions / changes are explicitly denoted, otherwise version 1 is assumed.

### Inputs (Client -> Robot)

| Name                            | Type          | Comment                                                                                  | Introduced in Version |
|---------------------------------|---------------|------------------------------------------------------------------------------------------|-----------------------|
| speed_slider_mask               | u32        | 0 = don't change speed slider with this input; 1 = use speed_slider_fraction to set speed slider value | |
| speed_slider_fraction           | f64        | New speed slider value                                                                   | |
| standard_digital_output_mask    | u8         | Standard digital output bit mask                                                         | |
| configurable_digital_output_mask| u8         | Configurable digital output bit mask                                                     | |
| standard_digital_output         | u8         | Standard digital outputs                                                                 | |
| configurable_digital_output     | u8         | Configurable digital outputs                                                             | |
| standard_analog_output_mask     | u8         | Bits 0-1: standard_analog_output_0, standard_analog_output_1                           | |
| standard_analog_output_type     | u8         | Output domain {0=current[mA], 1=voltage[V]}; Bits 0-1: standard_analog_output_0 \ standard_analog_output_1 | |
| standard_analog_output_0        | f64        | Standard analog output 0 (ratio) [0..1]                                                 | |
| standard_analog_output_1        | f64        | Standard analog output 1 (ratio) [0..1]                                                  | |
| input_bit_registers0_to_31      | u32        | This range of the boolean input registers is reserved for FieldBus/PLC interface usage.   | |
| input_bit_registers32_to_63     | u32        | This range of the boolean input registers is reserved for FieldBus/PLC interface usage.   | |
| input_bit_register_X            | bool          | 64 general purpose bits; X: [64..127] - The upper range of the boolean input registers can be used by external RTDE clients (i.e URCAPS). | |
| input_int_register_X            | i32         | 48 general purpose integer registers; X: [0..23] - The lower range of the integer input registers is reserved for FieldBus/PLC interface usage. [0..23] 3.4.0; X: [24..47] - The upper range of the integer input registers can be used by external RTDE clients (i.e URCAPS). [24..47] | 3.9.0 / 5.3.0 |
| input_double_register_X         | f64        | 48 general purpose double registers; X: [0..23] - The lower range of the double input registers is reserved for FieldBus/PLC interface usage. [0..23] 3.4.0; X: [24..47] - The upper range of the double input registers can be used by external RTDE clients (i.e URCAPS). [24..47] | 3.9.0 / 5.3.0 |
| external_force_torque           | VECTOR6D      | Input external wrench when using ft_rtde_input_enable builtin. | 3.3                        |

### Outputs (Robot -> Client)

| Name                                | Type          | Comment                                                                                                                                                     | Introduced Version    |
|-------------------------------------|---------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------------------|
| timestamp                           | f64        | Time elapsed since the controller was started [s]                                                                                                           |                       |
| target_q                            | VECTOR6D      | Target joint positions                                                                                                                                      |                       |
| target_qd                           | VECTOR6D      | Target joint velocities                                                                                                                                     |                       |
| target_qdd                          | VECTOR6D      | Target joint accelerations                                                                                                                                  |                       |
| target_current                      | VECTOR6D      | Target joint currents                                                                                                                                       |                       |
| target_moment                       | VECTOR6D      | Target joint moments (torques)                                                                                                                              |                       |
| actual_q                            | VECTOR6D      | Actual joint positions                                                                                                                                      |                       |
| actual_qd                           | VECTOR6D      | Actual joint velocities                                                                                                                                     |                       |
| actual_current                      | VECTOR6D      | Actual joint currents                                                                                                                                       |                       |
| joint_control_output                | VECTOR6D      | Joint control currents                                                                                                                                      |                       |
| actual_TCP_pose                     | VECTOR6D      | Actual Cartesian coordinates of the tool: (x,y,z,rx,ry,rz), where rx, ry and rz is a rotation vector representation of the tool orientation                 |                       |
| actual_TCP_speed                    | VECTOR6D      | Actual speed of the tool given in Cartesian coordinates. The speed is given in [m/s] and the rotational part of the TCP speed (rx, ry, rz) is the angular velocity given in [rad/s] | |
| actual_TCP_force                    | VECTOR6D      | Generalized forces in the TCP. It compensates the measurement for forces and torques generated by the payload                                               |                       |
| target_TCP_pose                     | VECTOR6D      | Target Cartesian coordinates of the tool: (x,y,z,rx,ry,rz), where rx, ry and rz is a rotation vector representation of the tool orientation                 |                       |
| target_TCP_speed                    | VECTOR6D      | Target speed of the tool given in Cartesian coordinates. The speed is given in [m/s] and the rotational part of the TCP speed (rx, ry, rz) is the angular velocity given in [rad/s] | |
| actual_digital_input_bits           | UINT64        | Current state of the digital inputs. 0-7: Standard, 8-15: Configurable, 16-17: Tool                                                                         |                       |
| joint_temperatures                  | VECTOR6D      | Temperature of each joint in degrees Celsius                                                                                                                |                       |
| actual_execution_time               | f64        | Controller real-time thread execution time                                                                                                                  |                       |
| robot_mode                          | i32         | Robot mode                                                                                                                                                  |                       |
| joint_mode                          | VECTOR6i32  | Joint control modes                                                                                                                                         |                       |
| safety_mode                         | i32         | Safety mode                                                                                                                                                 |                       |
| safety_status                       | i32         | Safety status                                                                                                                                               | 3.10.0 / 5.4.0        |
| actual_tool_accelerometer           | VECTOR3D      | Tool x, y and z accelerometer values                                                                                                                        |                       |
| speed_scaling                       | f64        | Speed scaling of the trajectory limiter                                                                                                                     |                       |
| target_speed_fraction               | f64        | Target speed fraction                                                                                                                                       |                       |
| actual_momentum                     | f64        | Norm of Cartesian linear momentum                                                                                                                           |                       |
| actual_main_voltage                 | f64        | Safety Control Board: Main voltage                                                                                                                          |                       |
| actual_robot_voltage                | f64        | Safety Control Board: Robot voltage (48V)                                                                                                                   |                       |
| actual_robot_current                | f64        | Safety Control Board: Robot current                                                                                                                         |                       |
| actual_joint_voltage                | VECTOR6D      | Actual joint voltages                                                                                                                                       |                       |
| actual_digital_output_bits          | UINT64        | Current state of the digital outputs. 0-7: Standard, 8-15: Configurable, 16-17: Tool                                                                        |                       |
| runtime_state                       | u32        | Program state                                                                                                                                               |                       |
| elbow_position                      | VECTOR3D      | Position of robot elbow in Cartesian Base Coordinates                                                                                                       | 3.5.0 / 5.0.0         |
| elbow_velocity                      | VECTOR3D      | Velocity of robot elbow in Cartesian Base Coordinates                                                                                                       | 3.5.0 / 5.0.0         |
| robot_status_bits                   | u32        | Bits 0-3: Is power on, Is program running, Is teach button pressed, Is power button pressed                                                                 |                       |
| safety_status_bits                  | u32        | Bits 0-10: Is normal mode, Is reduced mode, Is protective stopped, Is recovery mode, Is safeguard stopped, Is system emergency stopped, Is robot emergency stopped, Is emergency stopped, Is violation, Is fault, Is stopped due to safety | |
| analog_io_types                     | u32        | Bits 0-3: analog input 0, analog input 1, analog output 0, analog output 1, {0=current[A], 1=voltage[V]}                                                    |                       |
| standard_analog_input0              | f64        | Standard analog input 0 [A or V]                                                                                                                            |                       |
| standard_analog_input1              | f64        | Standard analog input 1 [A or V]                                                                                                                            |                       |
| standard_analog_output0             | f64        | Standard analog output 0 [A or V]                                                                                                                           |                       |
| standard_analog_output1             | f64        | Standard analog output 1 [A or V]                                                                                                                           |                       |
| io_current                          | f64        | I/O current [A]                                                                                                                                             |                       |
| euromap67_input_bits                | u32        | Euromap67 input bits                                                                                                                                        |                       |
| euromap67_output_bits               | u32        | Euromap67 output bits                                                                                                                                       |                       |
| euromap67_24V_voltage               | f64        | Euromap 24V voltage [V]                                                                                                                                     |                       |
| euromap67_24V_current               | f64        | Euromap 24V current [A]                                                                                                                                     |                       |
| tool_mode                           | u32        | Tool mode - Please see Remote Control Via TCP/IP - 16496                                                                                                    |                       |
| tool_analog_input_types             | u32        | Output domain {0=current[A], 1=voltage[V]} - Bits 0-1: tool_analog_input_0, tool_analog_input_1                                                             |                       |
| tool_analog_input0                  | f64        | Tool analog input 0 [A or V]                                                                                                                                |                       |
| tool_analog_input1                  | f64        | Tool analog input 1 [A or V]                                                                                                                                |                       |
| tool_output_voltage                 | i32         | Tool output voltage [V]                                                                                                                                     |                       |
| tool_output_current                 | f64        | Tool current [A]                                                                                                                                            |                       |
| tool_temperature                    | f64        | Tool temperature in degrees Celsius                                                                                                                         |                       |
| tcp_force_scalar                    | f64        | TCP force scalar [N]                                                                                                                                        |                       |
| output_bit_registers0_to_31         | u32        | General purpose bits                                                                                                                                        |                       |
| output_bit_registers32_to_63        | u32        | General purpose bits                                                                                                                                        |                       |
| output_bit_register_X               | bool          | 64 general purpose bits 3.9.0 / 5.3.0 - X: [64..127] - The upper range of the boolean output registers can be used by external RTDE clients (i.e URCAPS).   |                       |
| output_int_register_X               | i32         | 48 general purpose integer registers 3.9.0 / 5.3.0 - X: [0..23] - The lower range of the integer output registers is reserved for FieldBus/PLC interface usage. - X: [24..47] - The upper range of the integer output registers can be used by external RTDE clients (i.e URCAPS). | |
| output_double_register_X            | f64        | 48 general purpose double registers 3.9.0 / 5.3.0 - X: [0..23] - The lower range of the double output registers is reserved for FieldBus/PLC interface usage. - X: [24..47] - The upper range of the double output registers can be used by external RTDE clients (i.e URCAPS). | |
| input_bit_registers0_to_31          | u32        | General purpose bits - This range of the boolean output registers is reserved for FieldBus/PLC interface usage.                                             |                       |
| input_bit_registers32_to_63         | u32        | General purpose bits - This range of the boolean output registers is reserved for FieldBus/PLC interface usage.                                             |                       |
| input_bit_register_x                | bool          | 64 general purpose bits 3.9.0 / 5.3.0 - X: [64..127] - The upper range of the boolean output registers can be used by external RTDE clients (i.e URCAPS).   |                       |
| input_int_register_x ([0 .. 48])    | i32         | 48 general purpose integer registers 3.9.0 / 5.3.0 - X: [0..23] - The lower range of the integer input registers is reserved for FieldBus/PLC interface usage. - X: [24..47] - The upper range of the integer input registers can be used by external RTDE clients (i.e URCAPS). | |
| input_double_register_x ([0 .. 48]) | f64        | 48 general purpose double registers 3.9.0 / 5.3.0 - X: [0..23] - The lower range of the double input registers is reserved for FieldBus/PLC interface usage. - X: [24..47] - The upper range of the double input registers can be used by external RTDE clients (i.e URCAPS). | |
| tool_output_mode                    | u8         | The current output mode 5.2.0                                                                                                                               |                       |
| tool_digital_output0_mode           | u8         | The current mode of digital output 0 5.2.0                                                                                                                  |                       |
| tool_digital_output1_mode           | u8         | The current mode of digital output 1 5.2.0                                                                                                                  |                       |
| input_bit_registers0_to_31          | u32        | General purpose bits (input read back) 3.4.0                                                                                                                |                       |
| input_bit_registers32_to_63         | u32        | General purpose bits (input read back) 3.4.0                                                                                                                |                       |
| input_int_register_X                | i32         | 24 general purpose integer registers (input read back) 3.4.0 - X: [0..23]                                                                                   |                       |
| input_double_register_X             | f64        | 24 general purpose double registers (input read back) 3.4.0 - X: [0..23]                                                                                    |                       |
