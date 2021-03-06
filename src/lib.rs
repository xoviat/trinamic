#![no_std]
#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]
#![feature(min_type_alias_impl_trait)]

use core::convert::TryInto;
use core::future::Future;
use core::mem;

struct TMCHelpers {}

impl TMCHelpers {
    pub fn field_get(data: u32, mask: u32, shift: u32) -> u32 {
        (data & mask) >> shift
    }

    pub fn field_set(data: u32, mask: u32, shift: u32, value: u32) -> u32 {
        (data & (!mask)) | ((value << shift) & mask)
    }

    pub fn to_signed_32(x: u32) -> i32 {
        let m = x & 0xffffffff;
        ((m ^ 0x80000000) - 0x80000000).try_into().unwrap()
    }
}

struct TMCL {}

impl TMCL {
    //    @staticmethod
    //    def validate_host_id(host_id):
    //        if(not(type(host_id) == int)):
    //            raise TypeError
    //        if(not(0 <= host_id < 256)):
    //            raise ValueError("Incorrect Host ID value. Must be between 0 and 255 inclusively.")
    //
    //    @staticmethod
    //    def validate_module_id(module_id):
    //        if(not(type(module_id) == int)):
    //            raise TypeError
    //        if(not(0 <= module_id < 256)):
    //            raise ValueError("Incorrect Module ID value. Must be between 0 and 255 inclusively.")
    fn calculate_checksum(data: [u8; 8]) -> u8 {
        0
    }

    //    @staticmethod
    //    def calculate_checksum(data):
    //        checksum = 0
    //        for d in data:
    //            checksum += d
    //        checksum %= 256
    //        return checksum
}

pub trait TMCLConnnection {
    type SendFuture<'a>: Future<Output = ()>;
    type ReceiveFuture<'a>: Future<Output = [u8; 8]>;

    fn _send<'a>(&'a mut self, host_id: u16, module_id: u16, data: [u8; 8])
        -> Self::SendFuture<'a>;
    fn _recv<'a>(&'a mut self, host_id: u16, module_id: u16) -> Self::ReceiveFuture<'a>;
}

pub struct TMCLInterface<T: TMCLConnnection> {
    _MODULE_ID: u16,
    _HOST_ID: u16,
    connection: T,
}

impl<T: TMCLConnnection> TMCLInterface<T> {
    pub fn new(connection: T, host_id: Option<u16>, module_id: Option<u16>) -> Self {
        let host_id = match host_id {
            Some(host_id) => host_id,
            None => 2,
        };
        let module_id = match module_id {
            Some(module_id) => module_id,
            None => 1,
        };

        TMCLInterface {
            connection: connection,
            _MODULE_ID: module_id,
            _HOST_ID: host_id,
        }
    }

    //    """
    //    This class is a base class for sending TMCL commands over a communication
    //    interface.
    //    Each instance of this class represents one TMCL host. The bus connection for
    //    the TMCL communication is represented by a subclass inheriting this base
    //    class. An application with multiple busses should therefore use subclasses
    //    for all types of busses (e.g. one USB TMCL interface and one serial TMCL
    //    interface) and create exactly one instance of one of those subclasses per
    //    bus.
    //    A subclass is required to override the following functions:
    //        _send(self, hostID, module_id, data)
    //        _recv(self, hostID, module_id)
    //    A subclass may use the boolean _debug attribute to toggle printing further
    //    debug output.
    //    A subclass may read the _HOST_ID and _MODULE_ID parameters.
    //    """
    //
    //    pub fn __init__(&mut self, hostID=2, defaultmodule_id=1, debug=False) {
    //        """
    //        Parameters:
    //            hostID:
    //                Type: int, optional, default value: 2
    //                The ID of the TMCL host. This ID is the same for each module
    //                when communicating with multiple modules.
    //            defaultmodule_id:
    //                Type: int, optional, default value: 1
    //                The default module ID to use when no ID is given to any of the
    //                tmcl_interface functions. When only communicating with one
    //                module a script can omit the module_id for all TMCL interface
    //                calls by declaring this default value once at the start.
    //            debug:
    //                Type: bool, optional, default: False
    //                A switch for enabling debug mode. Can be changed with
    //                enableDebug(). In debug mode all sent and received TMCL packets
    //                get dumped to stdout. The boolean _debug attribute holds the
    //                current state of debug mode - subclasses may read it to print
    //                further debug output.
    //        """
    //
    //        TMCL.validate_host_id(hostID)
    //        TMCL.validate_module_id(defaultmodule_id)
    //
    //        if not type(debug) == bool:
    //            raise TypeError
    //
    //        self._HOST_ID    = hostID
    //        self._MODULE_ID  = defaultmodule_id
    //        self._debug      = debug
    //

    //    pub fn enableDebug(&mut self, enable) {
    //        """
    //        Set the debug mode, which dumps all TMCL datagrams written and read.
    //        """
    //        if type(enable) != bool:
    //            raise TypeError("Expected boolean value")
    //
    //        self._debug = enable
    //  }
    //
    //
    //    pub fn send_request(&mut self, request, module_id=None) {
    //        """
    //        Send a TMCL_Request and read back a TMCL_Reply. This function blocks until
    //        the reply has been received.
    //        """
    //
    //        if not module_id:
    //            module_id = self._MODULE_ID
    //
    //        if self._debug:
    //            request.dump()
    //
    //        self._send(self._HOST_ID, module_id, request.toBuffer())
    //        reply = TMCL_Reply.from_buffer(self._recv(self._HOST_ID, module_id))
    //
    //        if self._debug:
    //            reply.dump()
    //
    //        return reply
    //      }
    // Send a TMCL datagram and read back a reply. This function blocks until
    // the reply has been received.
    async fn send(
        &mut self,
        opcode: TMCLCommand,
        op_type: u8,
        motor: u8,
        value: i32,
        module_id: Option<u16>,
    ) -> TMCLReply {
        // If no module ID is given, use the default one
        let module_id = match module_id {
            None => self._MODULE_ID,
            Some(module_id) => module_id,
        };
        let request = TMCLRequest::new(module_id as u8, opcode, op_type, motor, value, None);

        self.connection
            ._send(self._HOST_ID, module_id, request.to_buffer())
            .await;

        TMCLReply::from_buffer(self.connection._recv(self._HOST_ID, module_id).await)
    }

    // Send the command for entering bootloader mode. This TMCL command does
    // sresult in a reply.
    //    async fn sendBoot(&mut self, module_id: Option<u16>) {
    //        // # If no module ID is given, use the default one
    //        let module_id = match module_id {
    //            None => self._MODULE_ID,
    //            Some(module_id) => module_id,
    //        };
    //
    //        let request = TMCLRequest::new(
    //            module_id as u8,
    //            TMCLCommand::BOOT,
    //            0x81,
    //            0x92,
    //            0xA3B4C5D6,
    //            None,
    //        );
    //
    //        // # Send the request
    //        self.connection
    //            ._send(self._HOST_ID, module_id, request.to_buffer())
    //            .await
    //    }
    //    pub fn getVersionString(&mut self, module_id=None) {
    //        """
    //        Request the ASCII version string.
    //        """
    //        reply = self.send(TMCLCommand.GET_FIRMWARE_VERSION, 0, 0, 0, module_id)
    //
    //        return reply.versionString()
    //
    //      }

    //    # General parameter access functions
    async fn parameter(
        &mut self,
        p_command: TMCLCommand,
        p_type: u8,
        p_axis: u8,
        p_value: i32,
        module_id: Option<u16>,
        signed: bool,
    ) -> i32 {
        let mut value = self
            .send(p_command, p_type, p_axis, p_value, module_id)
            .await
            .value as i32;
        if signed {
            value = TMCHelpers::to_signed_32(value as u32);
        }

        value
    }

    async fn set_parameter(
        &mut self,
        p_command: TMCLCommand,
        p_type: u8,
        p_axis: u8,
        p_value: i32,
        module_id: Option<u16>,
    ) -> TMCLReply {
        self.send(p_command, p_type, p_axis, p_value, module_id)
            .await
    }

    // Axis parameter access functions
    async fn axis_parameter(
        &mut self,
        command_type: u8,
        axis: u8,
        module_id: Option<u16>,
        signed: bool,
    ) -> i32 {
        let mut value = self
            .send(TMCLCommand::GAP, command_type, axis, 0, module_id)
            .await
            .value as i32;

        if signed {
            value = TMCHelpers::to_signed_32(value as u32);
        }

        value
    }

    async fn set_axis_parameter(
        &mut self,
        command_type: u8,
        axis: u8,
        value: i32,
        module_id: Option<u16>,
    ) -> TMCLReply {
        self.send(TMCLCommand::SAP, command_type, axis, value, module_id)
            .await
    }

    async fn store_axis_parameter(
        &mut self,
        command_type: u8,
        axis: u8,
        module_id: Option<u16>,
    ) -> TMCLReply {
        self.send(TMCLCommand::STAP, command_type, axis, 0, module_id)
            .await
    }

    async fn set_and_store_axis_parameter(
        &mut self,
        command_type: u8,
        axis: u8,
        value: i32,
        module_id: Option<u16>,
    ) {
        self.send(TMCLCommand::SAP, command_type, axis, value, module_id)
            .await;
        self.send(TMCLCommand::STAP, command_type, axis, 0, module_id)
            .await;
    }

    //    # Global parameter access functions
    async fn global_parameter(
        &mut self,
        command_type: u8,
        bank: u8,
        module_id: Option<u16>,
        signed: bool,
    ) -> i32 {
        let mut value = self
            .send(TMCLCommand::GGP, command_type, bank, 0, module_id)
            .await
            .value as i32;
        if signed {
            value = TMCHelpers::to_signed_32(value as u32);
        }

        value
    }

    async fn set_global_parameter(
        &mut self,
        command_type: u8,
        bank: u8,
        value: i32,
        module_id: Option<u16>,
    ) {
        self.send(TMCLCommand::SGP, command_type, bank, value, module_id)
            .await;
    }

    async fn store_global_parameter(&mut self, command_type: u8, bank: u8, module_id: Option<u16>) {
        self.send(TMCLCommand::STGP, command_type, bank, 0, module_id)
            .await;
    }

    async fn set_and_store_global_parameter(
        &mut self,
        command_type: u8,
        bank: u8,
        value: i32,
        module_id: Option<u16>,
    ) {
        self.send(TMCLCommand::SGP, command_type, bank, value, module_id)
            .await;
        self.send(TMCLCommand::STGP, command_type, bank, 0, module_id)
            .await;
    }

    // # Register access functions
    async fn write_mc(
        &mut self,
        register_address: u8,
        value: i32,
        module_id: Option<u16>,
    ) -> TMCLReply {
        self.write_register(register_address, TMCLCommand::WRITE_MC, 0, value, module_id)
            .await
    }

    async fn read_mc(&mut self, register_address: u8, module_id: Option<u16>, signed: bool) -> i32 {
        self.read_register(register_address, TMCLCommand::READ_MC, 0, module_id, signed)
            .await
    }

    async fn write_drv(
        &mut self,
        register_address: u8,
        value: i32,
        module_id: Option<u16>,
    ) -> TMCLReply {
        // return self.writeRegister(registerAddress, TMCLCommand.WRITE_DRV, 1, value, module_id);
        self.write_register(
            register_address,
            TMCLCommand::WRITE_DRV,
            1,
            value,
            module_id,
        )
        .await
    }

    async fn read_drv(
        &mut self,
        register_address: u8,
        module_id: Option<u16>,
        signed: bool,
    ) -> i32 {
        self.read_register(
            register_address,
            TMCLCommand::READ_DRV,
            1,
            module_id,
            signed,
        )
        .await
    }

    async fn read_register(
        &mut self,
        register_address: u8,
        command: TMCLCommand,
        channel: u8,
        module_id: Option<u16>,
        signed: bool,
    ) -> i32 {
        let mut value = self
            .send(command, register_address, channel, 0, module_id)
            .await
            .value as i32;

        if signed {
            value = TMCHelpers::to_signed_32(value as u32);
        }

        value
    }
    //
    async fn write_register(
        &mut self,
        register_address: u8,
        command: TMCLCommand,
        channel: u8,
        value: i32,
        module_id: Option<u16>,
    ) -> TMCLReply {
        self.send(command, register_address, channel, value, module_id)
            .await
    }

    //    # Motion control functions
    async fn rotate(&mut self, motor: u8, velocity: i32, module_id: Option<u16>) -> TMCLReply {
        self.send(TMCLCommand::ROR, 0, motor, velocity, module_id)
            .await
    }

    async fn stop(&mut self, motor: u8, module_id: Option<u16>) -> TMCLReply {
        self.send(TMCLCommand::MST, 0, motor, 0, module_id).await
    }

    async fn mv(
        &mut self,
        move_type: u8,
        motor: u8,
        position: i32,
        module_id: Option<u16>,
    ) -> TMCLReply {
        self.send(TMCLCommand::MVP, move_type, motor, position, module_id)
            .await
    }

    /*
        Use the TMCL MVP command to perform an absolute movement.
        Returns the value of the reply. Refer to the documentation of your
        specific module for details on what is returned.
    */
    async fn move_to(&mut self, motor: u8, position: i32, module_id: Option<u16>) -> u32 {
        self.mv(0, motor, position, module_id).await.value
    }
    //
    /*
        Use the TMCL MVP command to perform a relative movement.
        Returns the value of the reply. Refer to the documentation of your
        specific module for details on what is returned.
    */
    async fn move_by(&mut self, motor: u8, distance: i32, module_id: Option<u16>) -> u32 {
        self.mv(1, motor, distance, module_id).await.value
    }
    //
    //    # IO pin functions
    async fn analog_input(&mut self, x: u8, module_id: Option<u16>) -> u32 {
        self.send(TMCLCommand::GIO, x, 1, 0, module_id).await.value
    }
    //
    async fn digital_input(&mut self, x: u8, module_id: Option<u16>) -> u32 {
        self.send(TMCLCommand::GIO, x, 0, 0, module_id).await.value
    }
    //
    async fn digital_output(&mut self, x: u8, module_id: Option<u16>) -> u32 {
        self.send(TMCLCommand::GIO, x, 2, 0, module_id).await.value
    }
    //
    async fn set_digital_output(&mut self, x: u8, module_id: Option<u16>) -> u32 {
        self.send(TMCLCommand::SIO, x, 2, 1, module_id).await.value
    }
    //
    async fn clear_digital_output(&mut self, x: u8, module_id: Option<u16>) -> u32 {
        self.send(TMCLCommand::SIO, x, 2, 0, module_id).await.value
    }
    //
    //    " testing new interface usage (ED) => "
    //    # axis parameter access functions
    async fn axis_parameter_raw(
        &mut self,
        module_id: Option<u16>,
        axis: u8,
        command_type: u8,
    ) -> u32 {
        self.send(TMCLCommand::GAP, command_type, axis, 0, module_id)
            .await
            .value
    }
    //
    async fn set_axis_parameter_raw(
        &mut self,
        module_id: Option<u16>,
        axis: u8,
        command_type: u8,
        value: i32,
    ) -> TMCLReply {
        self.send(TMCLCommand::SAP, command_type, axis, value, module_id)
            .await
    }
    //
    //    # global parameter access functions
    async fn global_parameter_raw(
        &mut self,
        module_id: Option<u16>,
        bank: u8,
        command_type: u8,
    ) -> u32 {
        self.send(TMCLCommand::GGP, command_type, bank, 0, module_id)
            .await
            .value
    }
    //
    async fn set_global_parameter_raw(
        &mut self,
        module_id: Option<u16>,
        bank: u8,
        command_type: u8,
        value: i32,
    ) -> TMCLReply {
        self.send(TMCLCommand::SGP, command_type, bank, value, module_id)
            .await
    }
    //
    //    # IO pin functions
    async fn analog_input_raw(&mut self, module_id: Option<u16>, x: u8) -> u32 {
        self.send(TMCLCommand::GIO, x, 1, 0, module_id).await.value
    }
    //
    async fn digital_input_raw(&mut self, module_id: Option<u16>, x: u8) -> u32 {
        self.send(TMCLCommand::GIO, x, 0, 0, module_id).await.value
    }
    //
    async fn digital_output_raw(&mut self, module_id: Option<u16>, x: u8) -> u32 {
        self.send(TMCLCommand::GIO, x, 2, 0, module_id).await.value
    }
    //
    async fn set_digital_output_raw(&mut self, module_id: Option<u16>, x: u8) -> u32 {
        self.send(TMCLCommand::SIO, x, 2, 1, module_id).await.value
    }
    //
    async fn clear_digital_output_raw(&mut self, module_id: Option<u16>, x: u8) -> u32 {
        self.send(TMCLCommand::SIO, x, 2, 0, module_id).await.value
    }
    //
    //    " <= testing new interface usage (ED) "
}

#[allow(non_camel_case_types)]
pub enum TMCLCommand {
    ROR = 1,
    ROL = 2,
    MST = 3,
    MVP = 4,
    SAP = 5,
    GAP = 6,
    STAP = 7,
    RSAP = 8,
    SGP = 9,
    GGP = 10,
    STGP = 11,
    RSGP = 12,
    RFS = 13,
    SIO = 14,
    GIO = 15,
    CALC = 19,
    COMP = 20,
    JC = 21,
    JA = 22,
    CSUB = 23,
    RSUB = 24,
    WAIT = 27,
    STOP = 28,
    SAC = 29,
    SCO = 30,
    GCO = 31,
    CCO = 32,
    CALCX = 33,
    AAP = 34,
    AGP = 35,
    CLE = 36,
    TMCL_UF0 = 64,
    TMCL_UF1 = 65,
    TMCL_UF2 = 66,
    TMCL_UF3 = 67,
    TMCL_UF4 = 68,
    TMCL_UF5 = 69,
    TMCL_UF6 = 70,
    TMCL_UF7 = 71,
    STOP_APPLICATION = 128,
    RUN_APPLICATION = 129,
    STEP_APPLICATION = 130,
    RESET_APPLICATION = 131,
    START_DOWNLOAD_MODE = 132,
    QUIT_DOWNLOAD_MODE = 133,
    READ_TMCL_MEMORY = 134,
    GET_APPLICATION_STATUS = 135,
    GET_FIRMWARE_VERSION = 136,
    RESTORE_FACTORY_SETTINGS = 137,
    ASSIGNMENT = 143,
    WRITE_MC = 146,
    WRITE_DRV = 147,
    READ_MC = 148,
    READ_DRV = 149,
    BOOT_ERASE_ALL = 200,
    BOOT_WRITE_BUFFER = 201,
    BOOT_WRITE_PAGE = 202,
    BOOT_GET_CHECKSUM = 203,
    BOOT_READ_MEMORY = 204,
    BOOT_START_APPL = 205,
    BOOT_GET_INFO = 206,
    BOOT_WRITE_LENGTH = 208,
    BOOT = 242,
}

pub struct TMCLRequest {
    module_address: u8,
    command: u8,
    command_type: u8,
    motor_bank: u8,
    value: i32,
    checksum: u8,
}

impl TMCLRequest {
    pub fn new(
        address: u8,
        command: TMCLCommand,
        command_type: u8,
        motor_bank: u8,
        value: i32,
        checksum: Option<u8>,
    ) -> Self {
        let mut request = TMCLRequest {
            module_address: address & 0xFF,
            command: command as u8 & 0xFF,
            command_type: command_type & 0xFF,
            motor_bank: motor_bank & 0xFF,
            value: value,
            checksum: 0,
        };

        request.checksum = match checksum {
            Some(checksum) => checksum,
            None => TMCL::calculate_checksum(request.to_buffer()),
        };

        request
    }

    pub fn to_buffer(&self) -> [u8; 8] {
        [
            self.module_address,
            self.command,
            self.command_type,
            self.motor_bank,
            ((self.value >> (8 * 3)) & 0xff) as u8,
            ((self.value >> (8 * 2)) & 0xff) as u8,
            ((self.value >> (8 * 1)) & 0xff) as u8,
            ((self.value >> (8 * 0)) & 0xff) as u8,
        ]
    }
}

pub struct TMCLReply {
    reply_address: u8,
    module_address: u8,
    status: u8,
    command: u8,
    value: u32,
    checksum: u8,
    special: bool,
}

impl TMCLReply {
    pub fn new(
        reply_address: u8,
        module_address: u8,
        status: u8,
        command: TMCLCommand,
        value: u32,
        checksum: Option<u8>,
        special: bool,
    ) -> Self {
        TMCLReply {
            reply_address: reply_address & 0xFF,
            module_address: module_address & 0xFF,
            status: status & 0xFF,
            command: command as u8 & 0xFF,
            value: value & 0xFFFFFFFF,
            checksum: 0,
            special: special,
        }
    }

    pub fn from_buffer(data: [u8; 8]) -> Self {
        let reply_address: u8 = data[0];
        let module_address: u8 = data[1];
        let status: u8 = data[2];
        let command: TMCLCommand = unsafe { mem::transmute(data[3] as i8) };
        let value: u32 = ((data[4] as u32) << (8 * 3))
            + ((data[5] as u32) << (8 * 2))
            + ((data[6] as u32) << (8 * 1))
            + ((data[7] as u32) << (8 * 0));

        Self::new(
            reply_address,
            module_address,
            status,
            command,
            value,
            None,
            false,
        )
    }

    pub fn value(&self) -> u32 {
        self.value
    }

    pub fn is_valid(self) -> bool {
        self.status == 0
    }
}

pub mod connections;
pub mod modules;
