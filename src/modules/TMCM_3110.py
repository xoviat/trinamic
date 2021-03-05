'''
Created on 05.06.2020

@author: JM
'''

impl<T: TMCLConnnection> TMCM_3110<T> {
    MOTORS = 3

    pub fn new(connection: TMCLInterface<T>) -> Self {
        self.connection = connection

        #self.GPs   = _GPs
        self.APs   = _APs
        #self.ENUMs = _ENUMs

    @staticmethod
    def getEdsFile():
        return __file__.replace("TMCM_3110.py", "TMCM_3110_V.320.eds")

    def showChipInfo(self):
        ("The TMCM-3110 is a 3-Axis Stepper Controller / Driver. Voltage supply: 12 - 48V");

    # Axis parameter access
    pub async fn getAxisParameter(&mut self, apType, motor) {
        return self.connection.axisParameter(apType, motor)

    pub async fn setAxisParameter(&mut self, apType, motor, value) {
        self.connection.setAxisParameter(apType, motor, value)

    # Global parameter access
    pub async fn getGlobalParameter(&mut self, gpType, bank) {
        return self.connection.globalParameter(gpType, bank)

    pub async fn setGlobalParameter(&mut self, gpType, bank, value) {
        self.connection.setGlobalParameter(gpType, bank, value)

    # Motion Control functions
    pub async fn rotate(&mut self, motor, velocity) {
        self.setRampMode(motor, 2)

        self.setAxisParameter(self.APs.TargetVelocity, motor, velocity)

    pub async fn stop(&mut self, motor) {
        self.rotate(motor, 0)

    pub async fn moveTo(&mut self, motor, position, velocity=None) {
        if velocity:
            self.setMaxVelocity(motor, velocity)

        self.setTargetPosition(motor, position)

        self.setRampMode(motor, 0)

    pub async fn moveBy(&mut self, motor, difference, velocity=None) {
        position = difference + self.getActualPosition(motor)

        self.moveTo(motor, position, velocity)

        return position

    # Current control functions
    pub async fn setMotorRunCurrent(&mut self, motor, current) {
        self.setMaxCurrent(motor, current)

    pub async fn setMotorStandbyCurrent(&mut self, motor, current) {
        self.setAxisParameter(self.APs.StandbyCurrent, motor, current)

    pub async fn getMaxCurrent(&mut self, motor) {
        return self.getAxisParameter(self.APs.MaxCurrent, motor)

    pub async fn setMaxCurrent(&mut self, motor, current) {
        self.setAxisParameter(self.APs.MaxCurrent, motor, current)

    # StallGuard2 Functions
    pub async fn setStallguard2Filter(&mut self, motor, enableFilter) {
        self.setAxisParameter(self.APs.SG2FilterEnable, motor, enableFilter)

    pub async fn setStallguard2Threshold(&mut self, motor, threshold) {
        self.setAxisParameter(self.APs.SG2Threshold, motor, threshold)

    pub async fn setStopOnStallVelocity(&mut self, motor, velocity) {
        self.setAxisParameter(self.APs.smartEnergyStallVelocity, motor, velocity)

    # Motion parameter functions
    pub async fn getTargetPosition(&mut self, motor) {
        return self.getAxisParameter(self.APs.TargetPosition, motor)

    pub async fn setTargetPosition(&mut self, motor, position) {
        self.setAxisParameter(self.APs.TargetPosition, motor, position)

    pub async fn getActualPosition(&mut self, motor) {
        return self.getAxisParameter(self.APs.ActualPosition, motor)

    pub async fn setActualPosition(&mut self, motor, position) {
        return self.setAxisParameter(self.APs.ActualPosition, motor, position)

    pub async fn getTargetVelocity(&mut self, motor) {
        return self.getAxisParameter(self.APs.TargetVelocity, motor)

    pub async fn setTargetVelocity(&mut self, velocity, motor) {
        self.setAxisParameter(self.APs.TargetVelocity, motor, velocity)

    pub async fn getActualVelocity(&mut self, motor) {
        return self.getAxisParameter(self.APs.ActualVelocity, motor)

    pub async fn getMaxVelocity(&mut self, motor) {
        return self.getAxisParameter(self.APs.MaxVelocity, motor)

    pub async fn setMaxVelocity(&mut self, motor, velocity) {
        self.setAxisParameter(self.APs.MaxVelocity, motor, velocity)

    pub async fn getMaxAcceleration(&mut self, motor) {
        return self.getAxisParameter(self.APs.MaxAcceleration, motor)

    pub async fn setMaxAcceleration(&mut self, motor, acceleration) {
        self.setAxisParameter(self.APs.MaxAcceleration, motor, acceleration)

    pub async fn getRampMode(&mut self, motor) {
        return self.getAxisParameter(self.APs.RampMode, motor)

    pub async fn setRampMode(&mut self, motor, mode) {
        return self.setAxisParameter(self.APs.RampMode, motor, mode)

    # Status functions
    pub async fn getStatusFlags(&mut self, motor) {
        return self.getAxisParameter(self.APs.DrvStatusFlags, motor)

    pub async fn getErrorFlags(&mut self, motor) {
        return self.getAxisParameter(self.APs.ExtendedErrorFlags, motor)

    pub async fn positionReached(&mut self, motor) {
        return self.getAxisParameter(self.APs.PositionReachedFlag, motor)

    # IO pin functions
    pub async fn analogInput(&mut self, x) {
        return self.connection.analogInput(x)

    pub async fn digitalInput(&mut self, x) {
        return self.connection.digitalInput(x)

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
    ReferenceSwitchStatus          = 9
    RightEndstop                   = 10
    LeftEndstop                    = 11
    RightLimitSwitchDisable        = 12
    LeftLimitSwitchDisable         = 13
    MinimumSpeed                   = 130
    ActualAcceleration             = 135
    RampMode                       = 138
    MicrostepResolution            = 140
    ReferenceSwitchTolerance       = 141
    SoftStopFlag                   = 149
    EndSwitchPowerDown             = 150
    RampDivisor                    = 153
    PulseDivisor                   = 154
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
    SlopeControlHighSide           = 175
    SlopeControlLowSide            = 176
    ShortToGroundProtection        = 177
    ShortDetectionTime             = 178
    VSense                         = 179
    smartEnergyActualCurrent       = 180
    smartEnergyStallVelocity       = 181
    smartEnergyThresholdSpeed      = 182
    smartEnergySlowRunCurrent      = 183
    RandomTOffMode                 = 184
    ReferenceSearchMode            = 193
    ReferenceSearchSpeed           = 194
    ReferenceSwitchSpeed           = 195
    ReferenceSwitchDistance        = 196
    LastReferenceSwitchPosition    = 197
    BoostCurrent                   = 200
    EncoderMode                    = 201
    FreewheelingDelay              = 204
    LoadValue                      = 206
    ExtendedErrorFlags             = 207
    DrvStatusFlags                 = 208
    EncoderPosition                = 209
    EncoderPrescaler               = 210
    MaxEncoderDeviation            = 212
    GroupIndex                     = 213
    PowerDownDelay                 = 214
    StepDirectionMode              = 254
