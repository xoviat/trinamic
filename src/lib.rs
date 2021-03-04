use bxcan;
use bxcan::Id;
use bxcan::{Frame, StandardId};
use core::convert::TryInto;
use core::mem::transmute;
use nb::block;

struct TMCHelpers {}

impl TMCHelpers {
    pub fn field_get(data: u32, mask: u32, shift: u32) -> u32 {
        (data & mask) >> shift
    }

    pub fn field_set(data: u32, mask: u32, shift: u32, value: u32) -> u32 {
        (data & (!mask)) | ((value << shift) & mask)
    }

    pub fn to_signed_32(x: u32) -> u32 {
        let m = x & 0xffffffff;
        (m ^ 0x80000000) - 0x80000000
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

pub struct TMCLInterface<T: bxcan::Instance> {
    can: bxcan::Can<T>,
    _MODULE_ID: u16,
    _HOST_ID: u16,
}

impl<T: bxcan::Instance> TMCLInterface<T> {
    pub fn new(can: bxcan::Can<T>, host_id: Option<u16>, module_id: Option<u16>) -> Self {
        let host_id = match host_id {
            Some(host_id) => host_id,
            None => 2,
        };
        let module_id = match module_id {
            Some(module_id) => module_id,
            None => 1,
        };

        TMCLInterface {
            can: can,
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

    // Send the bytearray [data] representing a TMCL command. The length of
    // [data] is 9. The hostID and module_id parameters may be used for extended
    // addressing options available on the implemented communication interface.
    fn _send(&mut self, host_id: u16, module_id: u16, data: [u8; 8]) {
        let frame_tx = Frame::new_data(
            StandardId::new(module_id).unwrap(),
            [
                data[1], data[2], data[3], data[4], data[5], data[6], data[7],
            ],
        );

        block!(self.can.transmit(&frame_tx)).unwrap();
    }
    //
    // Receive a TMCL reply and return it as a bytearray. The length of the
    // returned byte array is 9. The hostID and module_id parameters may be used
    // for extended addressing options available on the implemented
    // communication interface.
    fn _recv(&mut self, host_id: u16, module_id: u16) -> [u8; 8] {
        let frame_rx = block!(self.can.receive()).unwrap();

        let id = match frame_rx.id() {
            Id::Extended(id) => id.as_raw() as u8,
            Id::Standard(id) => id.as_raw() as u8,
        };

        let data: [u8; 7] = match (frame_rx.data().unwrap() as &[u8]).try_into() {
            Ok(data) => data,
            Err(_) => [0; 7],
        };

        [
            id, data[0], data[1], data[2], data[3], data[4], data[5], data[6],
        ]
    }
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
    fn send(
        &mut self,
        opcode: TMCLCommand,
        opType: u8,
        motor: u8,
        value: u32,
        module_id: Option<u16>,
    ) -> TMCLReply {
        // If no module ID is given, use the default one
        let module_id = match module_id {
            None => self._MODULE_ID,
            Some(module_id) => module_id,
        };
        let request = TMCLRequest::new(module_id as u8, opcode, opType, motor, value, None);

        self._send(self._HOST_ID, module_id, request.to_buffer());

        TMCLReply::from_buffer(self._recv(self._HOST_ID, module_id))
    }

    // Send the command for entering bootloader mode. This TMCL command does
    // sresult in a reply.
    fn sendBoot(&mut self, module_id: Option<u16>) {
        // # If no module ID is given, use the default one
        let module_id = match module_id {
            None => self._MODULE_ID,
            Some(module_id) => module_id,
        };

        let request = TMCLRequest::new(
            module_id as u8,
            TMCLCommand::BOOT,
            0x81,
            0x92,
            0xA3B4C5D6,
            None,
        );

        // # Send the request
        self._send(self._HOST_ID, module_id, request.to_buffer())
    }
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
    fn parameter(
        &mut self,
        pCommand: TMCLCommand,
        pType: u8,
        pAxis: u8,
        pValue: u32,
        module_id: Option<u16>,
        signed: bool,
    ) -> u32 {
        let mut value = self.send(pCommand, pType, pAxis, pValue, module_id).value;
        if signed {
            value = TMCHelpers::to_signed_32(value);
        }

        value
    }

    fn setParameter(
        &mut self,
        pCommand: TMCLCommand,
        pType: u8,
        pAxis: u8,
        pValue: u32,
        module_id: Option<u16>,
    ) -> TMCLReply {
        self.send(pCommand, pType, pAxis, pValue, module_id)
    }

    // Axis parameter access functions
    fn axisParameter(
        &mut self,
        commandType: u8,
        axis: u8,
        module_id: Option<u16>,
        signed: bool,
    ) -> u32 {
        let mut value = self
            .send(TMCLCommand::GAP, commandType, axis, 0, module_id)
            .value;

        if signed {
            value = TMCHelpers::to_signed_32(value);
        }

        value
    }

    fn setAxisParameter(
        &mut self,
        commandType: u8,
        axis: u8,
        value: u32,
        module_id: Option<u16>,
    ) -> TMCLReply {
        self.send(TMCLCommand::SAP, commandType, axis, value, module_id)
    }

    fn storeAxisParameter(
        &mut self,
        commandType: u8,
        axis: u8,
        module_id: Option<u16>,
    ) -> TMCLReply {
        self.send(TMCLCommand::STAP, commandType, axis, 0, module_id)
    }

    fn setAndStoreAxisParameter(
        &mut self,
        commandType: u8,
        axis: u8,
        value: u32,
        module_id: Option<u16>,
    ) {
        self.send(TMCLCommand::SAP, commandType, axis, value, module_id);
        self.send(TMCLCommand::STAP, commandType, axis, 0, module_id);
    }

    //    # Global parameter access functions
    fn globalParameter(
        &mut self,
        commandType: u8,
        bank: u8,
        module_id: Option<u16>,
        signed: bool,
    ) -> u32 {
        let mut value = self
            .send(TMCLCommand::GGP, commandType, bank, 0, module_id)
            .value;
        if signed {
            value = TMCHelpers::to_signed_32(value);
        }

        value
    }

    fn setGlobalParameter(
        &mut self,
        commandType: u8,
        bank: u8,
        value: u32,
        module_id: Option<u16>,
    ) {
        self.send(TMCLCommand::SGP, commandType, bank, value, module_id);
    }

    fn storeGlobalParameter(&mut self, commandType: u8, bank: u8, module_id: Option<u16>) {
        self.send(TMCLCommand::STGP, commandType, bank, 0, module_id);
    }

    fn setAndStoreGlobalParameter(
        &mut self,
        commandType: u8,
        bank: u8,
        value: u32,
        module_id: Option<u16>,
    ) {
        self.send(TMCLCommand::SGP, commandType, bank, value, module_id);
        self.send(TMCLCommand::STGP, commandType, bank, 0, module_id);
    }

    // # Register access functions
    fn writeMC(&mut self, registerAddress: u8, value: u32, module_id: Option<u16>) -> TMCLReply {
        self.writeRegister(registerAddress, TMCLCommand::WRITE_MC, 0, value, module_id)
    }

    fn readMC(&mut self, registerAddress: u8, module_id: Option<u16>, signed: bool) -> u32 {
        self.readRegister(registerAddress, TMCLCommand::READ_MC, 0, module_id, signed)
    }

    fn writeDRV(&mut self, registerAddress: u8, value: u32, module_id: Option<u16>) -> TMCLReply {
        // return self.writeRegister(registerAddress, TMCLCommand.WRITE_DRV, 1, value, module_id);
        self.writeRegister(registerAddress, TMCLCommand::WRITE_DRV, 1, value, module_id)
    }

    fn readDRV(&mut self, registerAddress: u8, module_id: Option<u16>, signed: bool) -> u32 {
        self.readRegister(registerAddress, TMCLCommand::READ_DRV, 1, module_id, signed)
    }

    fn readRegister(
        &mut self,
        registerAddress: u8,
        command: TMCLCommand,
        channel: u8,
        module_id: Option<u16>,
        signed: bool,
    ) -> u32 {
        let mut value = self
            .send(command, registerAddress, channel, 0, module_id)
            .value;

        if signed {
            value = TMCHelpers::to_signed_32(value);
        }

        value
    }
    //
    fn writeRegister(
        &mut self,
        registerAddress: u8,
        command: TMCLCommand,
        channel: u8,
        value: u32,
        module_id: Option<u16>,
    ) -> TMCLReply {
        self.send(command, registerAddress, channel, value, module_id)
    }

    //    # Motion control functions
    fn rotate(&mut self, motor: u8, velocity: u32, module_id: Option<u16>) -> TMCLReply {
        self.send(TMCLCommand::ROR, 0, motor, velocity, module_id)
    }

    fn stop(&mut self, motor: u8, module_id: Option<u16>) -> TMCLReply {
        self.send(TMCLCommand::MST, 0, motor, 0, module_id)
    }

    fn mv(&mut self, moveType: u8, motor: u8, position: u32, module_id: Option<u16>) -> TMCLReply {
        self.send(TMCLCommand::MVP, moveType, motor, position, module_id)
    }

    /*
        Use the TMCL MVP command to perform an absolute movement.
        Returns the value of the reply. Refer to the documentation of your
        specific module for details on what is returned.
    */
    fn moveTo(&mut self, motor: u8, position: u32, module_id: Option<u16>) -> u32 {
        self.mv(0, motor, position, module_id).value
    }
    //
    /*
        Use the TMCL MVP command to perform a relative movement.
        Returns the value of the reply. Refer to the documentation of your
        specific module for details on what is returned.
    */
    fn moveBy(&mut self, motor: u8, distance: u32, module_id: Option<u16>) -> u32 {
        self.mv(1, motor, distance, module_id).value
    }
    //
    //    # IO pin functions
    fn analogInput(&mut self, x: u8, module_id: Option<u16>) -> u32 {
        self.send(TMCLCommand::GIO, x, 1, 0, module_id).value
    }
    //
    fn digitalInput(&mut self, x: u8, module_id: Option<u16>) -> u32 {
        self.send(TMCLCommand::GIO, x, 0, 0, module_id).value
    }
    //
    fn digitalOutput(&mut self, x: u8, module_id: Option<u16>) -> u32 {
        self.send(TMCLCommand::GIO, x, 2, 0, module_id).value
    }
    //
    fn setDigitalOutput(&mut self, x: u8, module_id: Option<u16>) -> u32 {
        self.send(TMCLCommand::SIO, x, 2, 1, module_id).value
    }
    //
    fn clearDigitalOutput(&mut self, x: u8, module_id: Option<u16>) -> u32 {
        self.send(TMCLCommand::SIO, x, 2, 0, module_id).value
    }
    //
    //    " testing new interface usage (ED) => "
    //    # axis parameter access functions
    fn axisParameterRaw(&mut self, module_id: Option<u16>, axis: u8, commandType: u8) -> u32 {
        self.send(TMCLCommand::GAP, commandType, axis, 0, module_id)
            .value
    }
    //
    fn setAxisParameterRaw(
        &mut self,
        module_id: Option<u16>,
        axis: u8,
        commandType: u8,
        value: u32,
    ) -> TMCLReply {
        self.send(TMCLCommand::SAP, commandType, axis, value, module_id)
    }
    //
    //    # global parameter access functions
    fn globalParameterRaw(&mut self, module_id: Option<u16>, bank: u8, commandType: u8) -> u32 {
        self.send(TMCLCommand::GGP, commandType, bank, 0, module_id)
            .value
    }
    //
    fn setGlobalParameterRaw(
        &mut self,
        module_id: Option<u16>,
        bank: u8,
        commandType: u8,
        value: u32,
    ) -> TMCLReply {
        self.send(TMCLCommand::SGP, commandType, bank, value, module_id)
    }
    //
    //    # IO pin functions
    fn analogInputRaw(&mut self, module_id: Option<u16>, x: u8) -> u32 {
        self.send(TMCLCommand::GIO, x, 1, 0, module_id).value
    }
    //
    fn digitalInputRaw(&mut self, module_id: Option<u16>, x: u8) -> u32 {
        self.send(TMCLCommand::GIO, x, 0, 0, module_id).value
    }
    //
    fn digitalOutputRaw(&mut self, module_id: Option<u16>, x: u8) -> u32 {
        self.send(TMCLCommand::GIO, x, 2, 0, module_id).value
    }
    //
    fn setDigitalOutputRaw(&mut self, module_id: Option<u16>, x: u8) -> u32 {
        self.send(TMCLCommand::SIO, x, 2, 1, module_id).value
    }
    //
    fn clearDigitalOutputRaw(&mut self, module_id: Option<u16>, x: u8) -> u32 {
        self.send(TMCLCommand::SIO, x, 2, 0, module_id).value
    }
    //
    //    " <= testing new interface usage (ED) "
}

pub struct TMCM_1276<T: bxcan::Instance> {
    connection: TMCLInterface<T>,
    MOTORS: u8,
    __default_motor: u8,
}

impl<T: bxcan::Instance> TMCM_1276<T> {
    pub fn new(connection: TMCLInterface<T>) -> Self {
        TMCM_1276 {
            connection: connection,
            MOTORS: 1,
            __default_motor: 0,
        }
    }

    //    pub fn __init__(&mut self, connection) {
    //        self.connection = connection
    //
    //        self.MOTORS = 1
    //        self.__default_motor = 0
    //
    //    @staticmethod
    //    def getEdsFile() {
    //        return __file__.replace("TMCM_1276.py", "TMCM_1276_V3.22.eds")
    //
    //    def showChipInfo(self) {
    //        ("The TMCM-1276 is a smart stepper motor driver module. The module is controlled via a CAN bus interface. Voltage supply: 10 - 30V");
    //
    //    # Axis parameter access
    pub fn getAxisParameter(&mut self, apType: _AP) -> u32 {
        self.connection
            .axisParameter(apType as u8, self.__default_motor, None, false)
    }
    //
    pub fn setAxisParameter(&mut self, apType: _AP, value: u32) -> TMCLReply {
        self.connection
            .setAxisParameter(apType as u8, self.__default_motor, value, None)
    }
    //
    //    # Global parameter access
    pub fn getGlobalParameter(&mut self, gpType: u8, bank: u8) -> u32 {
        self.connection.globalParameter(gpType, bank, None, false)
    }

    pub fn setGlobalParameter(&mut self, gpType: u8, bank: u8, value: u32) {
        self.connection
            .setGlobalParameter(gpType, bank, value, None)
    }
    //
    //    # Motion Control functions
    pub fn rotate(&mut self, velocity: u32) -> TMCLReply {
        self.setTargetVelocity(velocity)
    }
    //
    pub fn stop(&mut self) -> TMCLReply {
        self.rotate(0)
    }
    //
    pub fn moveTo(&mut self, position: u32, velocity: Option<u32>) {
        if velocity.is_some() {
            self.setMaxVelocity(velocity.unwrap());
        }

        self.connection.mv(0, self.__default_motor, position, None);
        self.setTargetPosition(position);
    }
    //
    pub fn moveBy(&mut self, difference: i32, velocity: Option<u32>) -> u32 {
        let position = difference + self.getActualPosition() as i32;
        self.moveTo(position as u32, velocity);

        position as u32
    }
    //
    //    # Current control functions
    pub fn setMotorRunCurrent(&mut self, current: u32) -> TMCLReply {
        self.setMaxCurrent(current)
    }
    //
    pub fn setMotorStandbyCurrent(&mut self, current: u32) -> TMCLReply {
        self.setAxisParameter(_AP::StandbyCurrent, current)
    }
    //
    pub fn getMaxCurrent(&mut self) -> u32 {
        self.getAxisParameter(_AP::MaxCurrent)
    }
    //
    pub fn setMaxCurrent(&mut self, current: u32) -> TMCLReply {
        self.setAxisParameter(_AP::MaxCurrent, current)
    }
    //
    //    # StallGuard2 Functions
    //    pub fn setStallguard2Filter(&mut self, enableFilter: u8) {
    //        self.setAxisParameter(_AP::StallGuard2FilterEnable, enableFilter)
    //    }
    //
    //    pub fn setStallguard2Threshold(&mut self, threshold) {
    //        self.setAxisParameter(_AP::StallGuard2Threshold, threshold)
    //
    //    pub fn setStopOnStallVelocity(&mut self, velocity) {
    //        self.setAxisParameter(_AP::StopOnStall, velocity)
    //
    //    # Motion parameter functions
    pub fn getTargetPosition(&mut self) -> u32 {
        self.getAxisParameter(_AP::TargetPosition)
    }
    //
    pub fn setTargetPosition(&mut self, position: u32) -> TMCLReply {
        self.setAxisParameter(_AP::TargetPosition, position)
    }
    //
    pub fn getActualPosition(&mut self) -> u32 {
        return self.getAxisParameter(_AP::ActualPosition);
    }
    //
    pub fn setActualPosition(&mut self, position: u32) -> TMCLReply {
        self.setAxisParameter(_AP::ActualPosition, position)
    }
    //
    pub fn getTargetVelocity(&mut self) -> u32 {
        self.getAxisParameter(_AP::TargetVelocity)
    }
    //
    pub fn setTargetVelocity(&mut self, velocity: u32) -> TMCLReply {
        self.setAxisParameter(_AP::TargetVelocity, velocity)
    }
    //
    pub fn getActualVelocity(&mut self) {
        self.getAxisParameter(_AP::ActualVelocity);
    }
    //
    pub fn getMaxVelocity(&mut self) -> u32 {
        self.getAxisParameter(_AP::MaxVelocity)
    }
    //
    pub fn setMaxVelocity(&mut self, velocity: u32) -> TMCLReply {
        self.setAxisParameter(_AP::MaxVelocity, velocity)
    }
    //
    pub fn getMaxAcceleration(&mut self) -> u32 {
        self.getAxisParameter(_AP::MaxAcceleration)
    }
    //
    pub fn setMaxAcceleration(&mut self, acceleration: u32) -> TMCLReply {
        self.setAxisParameter(_AP::MaxAcceleration, acceleration)
    }
    //
    //    pub fn getRampMode(self) -> u8 {
    //        self.getAxisParameter(_AP::RampMode)
    //    }
    //
    //    pub fn setRampMode(&mut self, mode) {
    //        return self.setAxisParameter(_AP::RampMode, mode)
    //
    //    # Status functions
    //    def getStatusFlags(self) {
    //        return self.getAxisParameter(_AP::TMC262ErrorFlags)
    //
    //    def getErrorFlags(self) {
    //        return self.getAxisParameter(_AP::ExtendedErrorFlags)
    //
    //    def positionReached(self) {
    //        return self.getAxisParameter(_AP::PositionReachedFlag)
    //
    //    # IO pin functions
    //    pub fn analogInput(&mut self, x) {
    //        return self.connection.analogInput(x)
    //
    //    pub fn digitalInput(&mut self, x) {
    //        return self.connection.digitalInput(x)
    //
    //    def showMotionConfiguration(self) {
    //        print("Motion configuration:")
    //        print("\tMax velocity: " + str(self.getMaxVelocity()))
    //        print("\tAcceleration: " + str(self.getMaxAcceleration()))
    //        print("\tRamp mode: " + ("position" if (self.getRampMode()==0) else "velocity"))
}

enum _AP {
    TargetPosition = 0,
    ActualPosition = 1,
    TargetVelocity = 2,
    ActualVelocity = 3,
    MaxVelocity = 4,
    MaxAcceleration = 5,
    MaxCurrent = 6,
    StandbyCurrent = 7,
    PositionReachedFlag = 8,
    HomeSwitch = 9,
    RightEndstop = 10,
    LeftEndstop = 11,
    AutomaticRightStop = 12,
    AutomaticLeftStop = 13,
    swapSwitchInputs = 14,
    A1 = 15,
    V1 = 16,
    MaxDeceleration = 17,
    D1 = 18,
    StartVelocity = 19,
    StopVelocity = 20,
    RampWaitTime = 21,
    THIGH = 22,
    VDCMIN = 23,
    rightSwitchPolarity = 24,
    leftSwitchPolarity = 25,
    softstop = 26,
    HighSpeedChopperMode = 27,
    HighSpeedFullstepMode = 28,
    MeasuredSpeed = 29,
    PowerDownRamp = 31,
    RelativePositioningOptionCode = 127,
    MicrostepResolution = 140,
    ChopperBlankTime = 162,
    ConstantTOffMode = 163,
    DisableFastDecayComparator = 164,
    ChopperHysteresisEnd = 165,
    ChopperHysteresisStart = 166,
    TOff = 167,
    SEIMIN = 168,
    SECDS = 169,
    smartEnergyHysteresis = 170,
    SECUS = 171,
    smartEnergyHysteresisStart = 172,
    SG2FilterEnable = 173,
    SG2Threshold = 174,
    ShortToGroundProtection = 177,
    smartEnergyActualCurrent = 180,
    smartEnergyStallVelocity = 181,
    smartEnergyThresholdSpeed = 182,
    RandomTOffMode = 184,
    ChopperSynchronization = 185,
    PWMThresholdSpeed = 186,
    PWMGrad = 187,
    PWMAmplitude = 188,
    PWMScale = 189,
    pwmMode = 190,
    PWMFrequency = 191,
    PWMAutoscale = 192,
    ReferenceSearchMode = 193,
    ReferenceSearchSpeed = 194,
    RefSwitchSpeed = 195,
    RightLimitSwitchPosition = 196,
    LastReferencePosition = 197,
    encoderMode = 201,
    MotorFullStepResolution = 202,
    pwmSymmetric = 203,
    FreewheelingMode = 204,
    LoadValue = 206,
    extendedErrorFlags = 207,
    DrvStatusFlags = 208,
    EncoderPosition = 209,
    EncoderResolution = 210,
    max_EncoderDeviation = 212,
    PowerDownDelay = 214,
    UnitMode = 255,
}

enum _GP {
    timer_0 = 0,
    timer_1 = 1,
    timer_2 = 2,
    stopLeft_0 = 27,
    stopRight_0 = 28,
    input_0 = 39,
    input_1 = 40,
    input_2 = 41,
    CANBitrate = 69,
    CANSendId = 70,
    CANReceiveId = 71,
    CANSecondaryId = 72,
    autoStartMode = 77,
    protectionMode = 81,
    eepromCoordinateStore = 84,
    zeroUserVariables = 85,
    applicationStatus = 128,
    programCounter = 130,
    lastTmclError = 131,
    tickTimer = 132,
    randomNumber = 133,
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
    moduleAddress: u8,
    command: u8,
    commandType: u8,
    motorBank: u8,
    value: u32,
    checksum: u8,
}

impl TMCLRequest {
    pub fn new(
        address: u8,
        command: TMCLCommand,
        commandType: u8,
        motorBank: u8,
        value: u32,
        checksum: Option<u8>,
    ) -> Self {
        let mut request = TMCLRequest {
            moduleAddress: address & 0xFF,
            command: command as u8 & 0xFF,
            commandType: commandType & 0xFF,
            motorBank: motorBank & 0xFF,
            value: value & 0xFFFFFFFF,
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
            self.moduleAddress,
            self.command,
            self.commandType,
            self.motorBank,
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
        let command: TMCLCommand = unsafe { transmute(data[3] as i8) };
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

    pub fn isValid(self) -> bool {
        self.status == 0
    }
}
