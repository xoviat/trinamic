use crate::{TMCLConnnection, TMCLInterface, TMCLReply};

pub struct TMCM_1276<T: TMCLConnnection> {
    connection: TMCLInterface<T>,
    MOTORS: u8,
    __default_motor: u8,
}

impl<T: TMCLConnnection> TMCM_1276<T> {
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
    pub async fn get_axis_parameter(&mut self, ap_type: _AP) -> i32 {
        self.connection
            .axis_parameter(ap_type as u8, self.__default_motor, None, false)
            .await
    }
    //
    pub async fn set_axis_parameter(&mut self, ap_type: _AP, value: i32) -> TMCLReply {
        self.connection
            .set_axis_parameter(ap_type as u8, self.__default_motor, value, None)
            .await
    }
    //
    //    # Global parameter access
    pub async fn get_global_parameter(&mut self, gp_type: u8, bank: u8) -> i32 {
        self.connection
            .global_parameter(gp_type, bank, None, false)
            .await
    }

    pub async fn set_global_parameter(&mut self, gp_type: u8, bank: u8, value: i32) {
        self.connection
            .set_global_parameter(gp_type, bank, value, None)
            .await
    }
    //
    //    # Motion Control functions
    pub async fn rotate(&mut self, velocity: i32) -> TMCLReply {
        self.set_target_velocity(velocity).await
    }
    //
    pub async fn stop(&mut self) -> TMCLReply {
        self.rotate(0).await
    }
    //
    pub async fn move_to(&mut self, position: i32, velocity: Option<i32>) {
        if velocity.is_some() {
            self.set_max_velocity(velocity.unwrap()).await;
        }

        self.connection
            .mv(0, self.__default_motor, position, None)
            .await;
        self.set_target_position(position).await;
    }
    //
    pub async fn move_by(&mut self, difference: i32, velocity: Option<i32>) -> u32 {
        let position = difference + self.get_actual_position().await;
        self.move_to(position, velocity).await;

        position as u32
    }
    //
    //    # Current control functions
    pub async fn set_motor_run_current(&mut self, current: i32) -> TMCLReply {
        self.set_max_current(current).await
    }
    //
    pub async fn set_motor_standby_current(&mut self, current: i32) -> TMCLReply {
        self.set_axis_parameter(_AP::StandbyCurrent, current).await
    }
    //
    pub async fn get_max_current(&mut self) -> i32 {
        self.get_axis_parameter(_AP::MaxCurrent).await
    }
    //
    pub async fn set_max_current(&mut self, current: i32) -> TMCLReply {
        self.set_axis_parameter(_AP::MaxCurrent, current).await
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
    pub async fn get_target_position(&mut self) -> i32 {
        self.get_axis_parameter(_AP::TargetPosition).await
    }
    //
    pub async fn set_target_position(&mut self, position: i32) -> TMCLReply {
        self.set_axis_parameter(_AP::TargetPosition, position).await
    }
    //
    pub async fn get_actual_position(&mut self) -> i32 {
        self.get_axis_parameter(_AP::ActualPosition).await
    }
    //
    pub async fn set_actual_position(&mut self, position: i32) -> TMCLReply {
        self.set_axis_parameter(_AP::ActualPosition, position).await
    }
    //
    pub async fn get_target_velocity(&mut self) -> i32 {
        self.get_axis_parameter(_AP::TargetVelocity).await
    }
    //
    pub async fn set_target_velocity(&mut self, velocity: i32) -> TMCLReply {
        self.set_axis_parameter(_AP::TargetVelocity, velocity).await
    }
    //
    pub async fn get_actual_velocity(&mut self) -> i32 {
        self.get_axis_parameter(_AP::ActualVelocity).await
    }
    //
    pub async fn get_max_velocity(&mut self) -> i32 {
        self.get_axis_parameter(_AP::MaxVelocity).await
    }
    //
    pub async fn set_max_velocity(&mut self, velocity: i32) -> TMCLReply {
        self.set_axis_parameter(_AP::MaxVelocity, velocity).await
    }
    //
    pub async fn get_max_acceleration(&mut self) -> i32 {
        self.get_axis_parameter(_AP::MaxAcceleration).await
    }
    //
    pub async fn set_max_acceleration(&mut self, acceleration: i32) -> TMCLReply {
        self.set_axis_parameter(_AP::MaxAcceleration, acceleration)
            .await
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

#[allow(non_camel_case_types)]
pub enum _AP {
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

#[allow(non_camel_case_types)]
pub enum _GP {
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
