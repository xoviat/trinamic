'''
Created on 21.09.2020

@author: AA
'''

impl<T: TMCLConnnection> TMCM_1240<T> {
    pub async fn __init__(&mut self, connection, moduleID=1) {
        self.connection = connection
        self.MODULE_ID  = moduleID

        self.GPs   = _GPs
        self.APs   = _APs
        self.ENUMs = _ENUMs

        self.MOTORS = 1
        self.__default_motor = 0

    def showChipInfo(self):
        ("The TMCM-1240 is a single axis controller/driver module. Voltage supply: 24V");

    " axis parameter access "
    pub async fn getAxisParameter(&mut self, apType, signed=False) {
        return self.connection.axisParameter(apType, self.__default_motor, self.MODULE_ID, signed)

    pub async fn setAxisParameter(&mut self, apType, value) {
        self.connection.setAxisParameter(apType, self.__default_motor, value, self.MODULE_ID)

    " global parameter access "
    pub async fn getGlobalParameter(&mut self, gpType, bank) {
        return self.connection.globalParameter(gpType, bank, self.MODULE_ID)

    pub async fn setGlobalParameter(&mut self, gpType, bank, value) {
        self.connection.setGlobalParameter(gpType, bank, value, self.MODULE_ID)

    " motion Control functions "
    pub async fn rotate(&mut self, velocity) {
        self.setAxisParameter(self.APs.TargetVelocity, velocity)

    def stop(self):
        self.rotate(0)

    pub async fn moveTo(&mut self, position, velocity=None) {
        if velocity:
            self.setMaxVelocity(velocity)

        return self.connection.moveTo(self.__default_motor, position, self.MODULE_ID)

    pub async fn moveBy(&mut self, difference, velocity=None) {
        if velocity:
            self.setMaxVelocity(velocity)

        return self.connection.moveBy(self.__default_motor, difference, self.MODULE_ID)

    " current control functions "
    pub async fn setMotorRunCurrent(&mut self, current) {
        self.setMaxCurrent(current)

    pub async fn setMotorStandbyCurrent(&mut self, current) {
        self.setAxisParameter(self.APs.StandbyCurrent, current)

    def getMaxCurrent(self):
        return self.getAxisParameter(self.APs.MaxCurrent)

    pub async fn setMaxCurrent(&mut self, current) {
        self.setAxisParameter(self.APs.MaxCurrent, current)

    " StallGuard2 functions "
    pub async fn setStallguard2Filter(&mut self, enableFilter) {
        self.setAxisParameter(self.APs.SG2FilterEnable, enableFilter)

    pub async fn setStallguard2Threshold(&mut self, threshold) {
        self.setAxisParameter(self.APs.SG2Threshold, threshold)

    pub async fn setStopOnStallVelocity(&mut self, velocity) {
        self.setAxisParameter(self.APs.smartEnergyStallVelocity, velocity)

    " motion parameter functions "
    def getTargetPosition(self):
        return self.getAxisParameter(self.APs.TargetPosition, signed=True)

    pub async fn setTargetPosition(&mut self, position) {
        self.setAxisParameter(self.APs.TargetPosition, position)

    def getActualPosition(self):
        return self.getAxisParameter(self.APs.ActualPosition, signed=True)

    pub async fn setActualPosition(&mut self, position) {
        return self.setAxisParameter(self.APs.ActualPosition, position)

    def getTargetVelocity(self):
        return self.getAxisParameter(self.APs.TargetVelocity, signed=True)

    pub async fn setTargetVelocity(&mut self, velocity) {
        self.setAxisParameter(self.APs.TargetVelocity, velocity)

    def getActualVelocity(self):
        return self.getAxisParameter(self.APs.ActualVelocity, signed=True)

    def getMaxVelocity(self):
        return self.getAxisParameter(self.APs.MaxVelocity)

    pub async fn setMaxVelocity(&mut self, velocity) {
        self.setAxisParameter(self.APs.MaxVelocity, velocity)

    def getMaxAcceleration(self):
        return self.getAxisParameter(self.APs.MaxAcceleration)

    pub async fn setMaxAcceleration(&mut self, acceleration) {
        self.setAxisParameter(self.APs.MaxAcceleration, acceleration)

    pub async fn setMicrostepResolution(&mut self, microstepResolution) {
        self.setAxisParameter(self.APs.MicrostepResolution, microstepResolution)

    " status functions "
    def getStatusFlags(self):
        return self.getAxisParameter(self.APs.DrvStatusFlags)

    def getErrorFlags(self):
        return self.getAxisParameter(self.APs.ExtendedErrorFlags)

    def positionReached(self):
        return self.getAxisParameter(self.APs.PositionReachedFlag)

    " IO pin functions "
    pub async fn analogInput(&mut self, x) {
        return self.connection.analogInput(x, self.MODULE_ID)

    pub async fn digitalInput(&mut self, x) {
        return self.connection.digitalInput(x, self.MODULE_ID)

pub enum _AP {
    TargetPosition                 = 0
    ActualPosition                 = 1
    TargetVelocity                 = 2
    ActualVelocity                 = 3
    MaxVelocity                    = 4
    MaxAcceleration                = 5
    MaxCurrent                     = 6
    StandbyCurrent                 = 7
    PositionReachedFlag            = 8
    HomeSwitch                     = 9
    RightEndstop                   = 10
    LeftEndstop                    = 11
    RightLimitSwitchDisable        = 12
    LeftLimitSwitchDisable         = 13
    SwapLimitSwitches              = 14
    A1                             = 15
    V1                             = 16
    MaxDeceleration                = 17
    D1                             = 18
    StartVelocity                  = 19
    StopVelocity                   = 20
    RampWaitTime                   = 21
    THIGH                          = 22
    min_dcStep_speed               = 23
    RightLimitSwitchPolarity       = 24
    LeftLimitSwitchPolarity        = 25
    softstop                       = 26
    HighSpeedChopperMode           = 27
    HighSpeedFullstepMode          = 28
    MeasuredSpeed                  = 29
    PowerDownRamp                  = 31
    dcStep_time                    = 32
    dcStep_stallGuard              = 33
    relative_positioning_option    = 127
    MicrostepResolution            = 140
    Intpol                         = 160
    DoubleEdgeSteps                = 161
    ChopperBlankTime               = 162
    ConstantTOffMode               = 163
    DisableFastDecayComparator     = 164
    ChopperHysteresisEnd           = 165
    ChopperHysteresisStart         = 166
    TOff                           = 167
    SEIMIN                         = 168
    SECDS                          = 169
    smartEnergyHysteresis          = 170
    SECUS                          = 171
    smartEnergyHysteresisStart     = 172
    SG2FilterEnable                = 173
    SG2Threshold                   = 174
    GlobalCurrentScaler            = 178
    smartEnergyActualCurrent       = 180
    smartEnergyStallVelocity       = 181
    smartEnergyThresholdSpeed      = 182
    RandomTOffMode                 = 184
    ChopperSynchronization         = 185
    PWMThresholdSpeed              = 186
    PWMGrad                        = 187
    PWMAmplitude                   = 188
    PWMScale                       = 189
    PWMMode                        = 190
    PWMFrequency                   = 191
    PWMAutoscale                   = 192
    ReferenceSearchMode            = 193
    ReferenceSearchSpeed           = 194
    ReferenceSwitchSpeed           = 195
    ReferenceSwitchDistance        = 196
    LastReferenceSwitchPosition    = 197
    FullstepResolution             = 202
    FreewheelingMode               = 204
    LoadValue                      = 206
    ExtendedErrorFlags             = 207
    DrvStatusFlags                 = 208
    EncoderPosition                = 209
    encoder_cleat                  = 210
    max_encoder_deviation          = 212
    PowerDownDelay                 = 214
    absolute_resolver_value        = 215
    external_encoder_position      = 216
    external_encoder_resolution    = 217
    external_encoder_max_deviation = 218
    reverse_shaft                  = 251
    step_direction_mode            = 254
    unit_mode                      = 255

class _ENUMs():
    FLAG_POSITION_END = 0x00004000

pub enum _GP {
    timer_0                        = 0
    timer_1                        = 1
    timer_2                        = 2
    stopLeft_0                     = 27
    stopRight_0                    = 28
    input_0                        = 39
    input_1                        = 40
    input_2                        = 41
    input_3                        = 42
    serialBaudRate                 = 65
    serialAddress                  = 66
    ASCIIMode                      = 67           
    serialHeartbeat                = 68
    telegramPauseTime              = 75
    serialHostAddress              = 76
    autoStartMode                  = 77
    limitSwitchPolarity            = 79             
    protectionMode                 = 81
    eepromCoordinateStore          = 84
    zeroUserVariables              = 85
    serialSecondaryAddress         = 87
    reverseShaftDirection          = 90            
    applicationStatus              = 128
    downloadMode                   = 129
    programCounter                 = 130
    lastTmclError                  = 131        
    tickTimer                      = 132
    randomNumber                   = 133
    Intpol                         = 255
