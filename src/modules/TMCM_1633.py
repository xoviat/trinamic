'''
Created on 25.06.2019

@author: ED
'''
from PyTrinamic.helpers import TMC_helpers

impl<T: TMCLConnnection> TMCM_1633<T> {
    pub fn new(connection: TMCLInterface<T>) -> Self {
        self.connection = connection

        self.GPs   = _GPs
        self.APs   = _APs
        self.ENUMs = _ENUMs

        self.motor = 0

    @staticmethod
    def getEdsFile():
        return __file__.replace("TMCM_1633.py", "TMCM_1633_Hw1.00_Fw2.10.eds")

    def showChipInfo(self):
        ("The TMCM-1633 is a highly integrated single axis BLDC servo controller module with several interface options. Voltage supply: 14,5 - 48V");

    " axis parameter access "
    pub async fn axisParameter(&mut self, apType) {
        return self.connection.axisParameter(apType, self.motor)

    pub async fn setAxisParameter(&mut self, apType, value) {
        self.connection.setAxisParameter(apType, self.motor, value)

    " global parameter access "
    pub async fn globalParameter(&mut self, gpType) {
        return self.connection.globalParameter(gpType, self.motor)

    pub async fn setGlobalParameter(&mut self, gpType, value) {
        self.connection.setGlobalParameter(gpType, self.motor, value)

    " standard functions "
    pub async fn moveToPosition(&mut self, position) {
        self.setAxisParameter(self.APs.TargetPosition, position)

    def targetPosition(self):
        return self.axisParameter(self.APs.TargetPosition)

    def actualPosition(self):
        return TMC_helpers.toSigned32(self.axisParameter(self.APs.ActualPosition))    

    pub async fn setActualPosition(&mut self, position) {
        return self.setAxisParameter(self.APs.ActualPosition, position)

    pub async fn rotate(&mut self, velocity) {
        self.setAxisParameter(self.APs.TargetVelocity, velocity)

    def actualVelocity(self):
        return TMC_helpers.toSigned32(self.axisParameter(self.APs.ActualVelocity))    

    pub async fn setTargetTorque(&mut self, torque) {
        self.setAxisParameter(self.APs.TargetTorque, torque)

    def targetTorque(self):
        return self.axisParameter(self.APs.TargetTorque)

    def actualTorque(self):
        return self.axisParameter(self.APs.ActualTorque)

    " helpful functions "

    def maxVelocity(self):
        return self.axisParameter(self.APs.MaxVelocity)

    pub async fn setMaxVelocity(&mut self, maxVelocity) {
        self.setAxisParameter(self.APs.MaxVelocity, maxVelocity)

    def maxTorque(self):
        return self.axisParameter(self.APs.MaxTorque)

    pub async fn setMaxTorque(&mut self, maxTorque) {
        self.setAxisParameter(self.APs.MaxTorque, maxTorque)

    def openLoopTorque(self):
        return self.axisParameter(self.APs.StartCurrent)

    pub async fn setOpenLoopTorque(&mut self, torque) {
        self.setAxisParameter(self.APs.StartCurrent, torque)

    def acceleration(self):
        return self.axisParameter(self.APs.Acceleration)

    pub async fn setAcceleration(&mut self, acceleration) {
        self.setAxisParameter(self.APs.Acceleration, acceleration)

    def targetReachedVelocity(self):
        return self.axisParameter(self.APs.TargetReachedVelocity)

    pub async fn setTargetReachedVelocity(&mut self, velocity) {
        self.setAxisParameter(self.APs.TargetReachedVelocity, velocity)

    def targetReachedDistance(self):
        return self.axisParameter(self.APs.TargetReachedDistance)

    pub async fn setTargetReachedDistance(&mut self, distance) {
        self.setAxisParameter(self.APs.TargetReachedDistance, distance)

    def motorHaltedVelocity(self):
        return self.axisParameter(self.APs.MotorHaltedVelocity)

    pub async fn setMotorHaltedVelocity(&mut self, velocity) {
        self.setAxisParameter(self.APs.MotorHaltedVelocity, velocity)

    def positionReached(self):
        return ((self.statusFlags() & self.ENUMs.FLAG_POSITION_END) != 0)

    def rampEnabled(self):
        return self.axisParameter(self.APs.EnableRamp)

    pub async fn setRampEnabled(&mut self, enable) {
        self.setAxisParameter(self.APs.EnableRamp, enable)

    def torquePParameter(self):
        return self.axisParameter(self.APs.TorqueP)

    pub async fn setTorquePParameter(&mut self, pValue) {
        self.setAxisParameter(self.APs.TorqueP, pValue)

    def torqueIParameter(self):
        return self.axisParameter(self.APs.TorqueI)

    pub async fn setTorqueIParameter(&mut self, pValue) {
        self.setAxisParameter(self.APs.TorqueI, pValue)

    def velocityPParameter(self):
        return self.axisParameter(self.APs.VelocityP)

    pub async fn setVelocityPParameter(&mut self, pValue) {
        self.setAxisParameter(self.APs.VelocityP, pValue)

    def velocityIParameter(self):
        return self.axisParameter(self.APs.VelocityI)

    pub async fn setVelocityIParameter(&mut self, pValue) {
        self.setAxisParameter(self.APs.VelocityI, pValue)

    def positionPParameter(self):
        return self.axisParameter(self.APs.PositionP)

    pub async fn setPositionPParameter(&mut self, pValue) {
        self.setAxisParameter(self.APs.PositionP, pValue)

    def motorPoles(self):
        return self.axisParameter(self.APs.MotorPoles)

    pub async fn setMotorPoles(&mut self, poles) {
        self.setAxisParameter(self.APs.MotorPoles, poles)

    def hallInvert(self):
        return self.axisParameter(self.APs.HallSensorInvert)

    pub async fn setHallInvert(&mut self, invert) {
        self.setAxisParameter(self.APs.HallSensorInvert, invert)

    def encoderInitMode(self):
        return self.axisParameter(self.APs.EncoderInitMode)

    pub async fn setEncoderInitMode(&mut self, mode) {
        self.setAxisParameter(self.APs.EncoderInitMode, mode)

    def encoderResolution(self):
        return self.axisParameter(self.APs.EncoderSteps)

    pub async fn setEncoderResolution(&mut self, steps) {
        self.setAxisParameter(self.APs.EncoderSteps, steps)

    def encoderDirection(self):
        return self.axisParameter(self.APs.EncoderDirection)

    pub async fn setEncoderDirection(&mut self, direction) {
        self.setAxisParameter(self.APs.EncoderDirection, direction)

    def commutationMode(self):
        return self.axisParameter(self.APs.CommutationMode)

    pub async fn setCommutationMode(&mut self, mode) {
        self.setAxisParameter(self.APs.CommutationMode, mode)

    def statusFlags(self):
        return self.axisParameter(self.APs.StatusFlags)

    pub async fn analogInput(&mut self, x) {
        return self.connection.analogInput(x)

    pub async fn digitalInput(&mut self, x) {
        return self.connection.digitalInput(x)

    pub async fn digitalOutput(&mut self, x) {
        return self.connection.digitalOutput(x)

    pub async fn setDigitalOutput(&mut self, x) {
        return self.connection.setDigitalOutput(x, 1)

    pub async fn clearDigitalOutput(&mut self, x) {
        return self.connection.setDigitalOutput(x, 0)

    def showMotorConfiguration(self):
        print("Motor configuration:")
        print("\tMotor poles: " + str(self.motorPoles()))
        print("\tMax torque:  " + str(self.maxTorque()) + " mA")

    def showHallConfiguration(self):
        print("Hall configuration:")
        print("\tHall invert: " + str(self.hallInvert()))

    def showEncoderConfiguration(self):
        print("Encoder configuration:")
        print("\tOpen loop torque:   " + str(self.openLoopTorque()) + " mA")
        print("\tEncoder resolution: " + str(self.encoderResolution()))
        print("\tEncoder direction:  " + str(self.encoderDirection()))
        print("\tEncoder init mode:  " + str(self.encoderInitMode()))

    def showMotionConfiguration(self):
        print("Motion configuration:")
        print("\tMax velocity: " + str(self.maxVelocity()))
        print("\tAcceleration: " + str(self.acceleration()))
        print("\tRamp enabled: " + ("disabled" if (self.rampEnabled()==0) else "enabled"))
        print("\tMotor halted velocity:   " + str(self.motorHaltedVelocity()))
        print("\tTarget reached velocity: " + str(self.targetReachedVelocity()))
        print("\tTarget reached distance: " + str(self.targetReachedDistance()))

    def showPIConfiguration(self):
        print("PI configuration:")
        print("\tTorque   P: " + str(self.torquePParameter()) + " I: " + str(self.torqueIParameter()))
        print("\tVelocity P: " + str(self.velocityPParameter()) + " I: " + str(self.velocityIParameter()))
        print("\tPosition P: " + str(self.positionPParameter()))

pub enum _AP {
    TargetPosition                 = 0
    ActualPosition                 = 1
    TargetVelocity                 = 2
    ActualVelocity                 = 3
    MaxVelocity                    = 4
    MaxTorque                      = 6
    TargetReachedVelocity          = 7
    MotorHaltedVelocity            = 9
    TargetReachedDistance          = 10
    Acceleration                   = 11
    RampVelocity                   = 13
    ReinitBldcRegulation           = 31
    PIDRegulationLoopDelay         = 133
    CurrentRegulationLoopDelay     = 134
    EnableRamp                     = 146
    ActualTorque                   = 150
    SupplyVoltage                  = 151
    DriverTemperature              = 152
    TargetTorque                   = 155
    StatusFlags                    = 156
    CommutationMode                = 159
    ClearOnNull                    = 161
    ClearOnce                      = 163
    EncoderOffset                  = 165
    TorqueP                        = 172
    TorqueI                        = 173
    StartCurrent                   = 177
    MainLoopsPerSecond             = 180
    PwmLoopsPerSecond              = 181
    TorqueLoopsPerSecond           = 182
    VelocityLoopsPerSecond         = 183
    DebugValue0                    = 190
    DebugValue1                    = 191
    DebugValue2                    = 192
    DebugValue3                    = 193
    DebugValue4                    = 194
    DebugValue5                    = 195
    DebugValue6                    = 196
    DebugValue7                    = 197
    DebugValue8                    = 198
    DebugValue9                    = 199
    CurrentPIDError                = 200
    CurrentPIDErrorSum             = 201
    ActualHallAngle                = 210
    ActualEncoderAngle             = 211
    ActualControlledAngle          = 212
    PositionPIDError               = 226
    VelocityPIDError               = 228
    VelocityPIDErrorSum            = 229
    PositionP                      = 230
    VelocityP                      = 234
    VelocityI                      = 235
    InitVelocity                   = 241
    InitSineDelay                  = 244
    EncoderInitMode                = 249
    EncoderSteps                   = 250
    EncoderDirection               = 251
    HallInterpolation              = 252
    MotorPoles                     = 253
    HallSensorInvert               = 254
    DriverEnabled                  = 255

class _ENUMs():
    COMM_MODE_FOC_HALL              = 6
    COMM_MODE_FOC_ENCODER           = 7
    COMM_MODE_FOC_CONTROLLED        = 8

    ENCODER_INIT_MODE_0             = 0
    ENCODER_INIT_MODE_1             = 1
    ENCODER_INIT_MODE_2             = 2

    FLAG_POSITION_END               = 0x00004000

pub enum _GP {
    serialBaudRate                 = 65
    serialAddress                  = 66
    CANBitRate                     = 69
    CANsendID                      = 70
    CANreceiveID                   = 71
    telegramPauseTime              = 75
    serialHostAddress              = 76
    autoStartMode                  = 77
    applicationStatus              = 128
    programCounter                 = 130
    tickTimer                      = 132
