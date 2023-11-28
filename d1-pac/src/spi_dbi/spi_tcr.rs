#[doc = "Register `spi_tcr` reader"]
pub type R = crate::R<SPI_TCR_SPEC>;
#[doc = "Register `spi_tcr` writer"]
pub type W = crate::W<SPI_TCR_SPEC>;
#[doc = "Field `cpha` reader - SPI Clock/Data Phase Control"]
pub type CPHA_R = crate::BitReader<CPHA_A>;
#[doc = "SPI Clock/Data Phase Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPHA_A {
    #[doc = "0: Phase 0 (Leading edge for sample data)"]
    P0 = 0,
    #[doc = "1: Phase 1 (Leading edge for setup data)"]
    P1 = 1,
}
impl From<CPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CPHA_A) -> Self {
        variant as u8 != 0
    }
}
impl CPHA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CPHA_A {
        match self.bits {
            false => CPHA_A::P0,
            true => CPHA_A::P1,
        }
    }
    #[doc = "Phase 0 (Leading edge for sample data)"]
    #[inline(always)]
    pub fn is_p0(&self) -> bool {
        *self == CPHA_A::P0
    }
    #[doc = "Phase 1 (Leading edge for setup data)"]
    #[inline(always)]
    pub fn is_p1(&self) -> bool {
        *self == CPHA_A::P1
    }
}
#[doc = "Field `cpha` writer - SPI Clock/Data Phase Control"]
pub type CPHA_W<'a, REG> = crate::BitWriter<'a, REG, CPHA_A>;
impl<'a, REG> CPHA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Phase 0 (Leading edge for sample data)"]
    #[inline(always)]
    pub fn p0(self) -> &'a mut crate::W<REG> {
        self.variant(CPHA_A::P0)
    }
    #[doc = "Phase 1 (Leading edge for setup data)"]
    #[inline(always)]
    pub fn p1(self) -> &'a mut crate::W<REG> {
        self.variant(CPHA_A::P1)
    }
}
#[doc = "Field `cpol` reader - SPI Clock Polarity Control"]
pub type CPOL_R = crate::BitReader<CPOL_A>;
#[doc = "SPI Clock Polarity Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPOL_A {
    #[doc = "0: Active high polarity"]
    HIGH = 0,
    #[doc = "1: Active low polarity"]
    LOW = 1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl CPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::HIGH,
            true => CPOL_A::LOW,
        }
    }
    #[doc = "Active high polarity"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CPOL_A::HIGH
    }
    #[doc = "Active low polarity"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CPOL_A::LOW
    }
}
#[doc = "Field `cpol` writer - SPI Clock Polarity Control"]
pub type CPOL_W<'a, REG> = crate::BitWriter<'a, REG, CPOL_A>;
impl<'a, REG> CPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active high polarity"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(CPOL_A::HIGH)
    }
    #[doc = "Active low polarity"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(CPOL_A::LOW)
    }
}
#[doc = "Field `spol` reader - SPI Chip Select Signal Polarity Control"]
pub type SPOL_R = crate::BitReader<SPOL_A>;
#[doc = "SPI Chip Select Signal Polarity Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPOL_A {
    #[doc = "0: Active high polarity"]
    HIGH = 0,
    #[doc = "1: Active low polarity"]
    LOW = 1,
}
impl From<SPOL_A> for bool {
    #[inline(always)]
    fn from(variant: SPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl SPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPOL_A {
        match self.bits {
            false => SPOL_A::HIGH,
            true => SPOL_A::LOW,
        }
    }
    #[doc = "Active high polarity"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SPOL_A::HIGH
    }
    #[doc = "Active low polarity"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SPOL_A::LOW
    }
}
#[doc = "Field `spol` writer - SPI Chip Select Signal Polarity Control"]
pub type SPOL_W<'a, REG> = crate::BitWriter<'a, REG, SPOL_A>;
impl<'a, REG> SPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active high polarity"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(SPOL_A::HIGH)
    }
    #[doc = "Active low polarity"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(SPOL_A::LOW)
    }
}
#[doc = "Field `ssctl` reader - "]
pub type SSCTL_R = crate::BitReader<SSCTL_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSCTL_A {
    #[doc = "0: SPI_SSx remains asserted between SPI bursts"]
    ASSERT = 0,
    #[doc = "1: Negate SPI_SSx between SPI bursts"]
    NEGATE = 1,
}
impl From<SSCTL_A> for bool {
    #[inline(always)]
    fn from(variant: SSCTL_A) -> Self {
        variant as u8 != 0
    }
}
impl SSCTL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSCTL_A {
        match self.bits {
            false => SSCTL_A::ASSERT,
            true => SSCTL_A::NEGATE,
        }
    }
    #[doc = "SPI_SSx remains asserted between SPI bursts"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == SSCTL_A::ASSERT
    }
    #[doc = "Negate SPI_SSx between SPI bursts"]
    #[inline(always)]
    pub fn is_negate(&self) -> bool {
        *self == SSCTL_A::NEGATE
    }
}
#[doc = "Field `ssctl` writer - "]
pub type SSCTL_W<'a, REG> = crate::BitWriter<'a, REG, SSCTL_A>;
impl<'a, REG> SSCTL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI_SSx remains asserted between SPI bursts"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut crate::W<REG> {
        self.variant(SSCTL_A::ASSERT)
    }
    #[doc = "Negate SPI_SSx between SPI bursts"]
    #[inline(always)]
    pub fn negate(self) -> &'a mut crate::W<REG> {
        self.variant(SSCTL_A::NEGATE)
    }
}
#[doc = "Field `ss_sel` reader - "]
pub type SS_SEL_R = crate::FieldReader<SS_SEL_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SS_SEL_A {
    #[doc = "0: `0`"]
    SS0 = 0,
    #[doc = "1: `1`"]
    SS1 = 1,
    #[doc = "2: `10`"]
    SS2 = 2,
    #[doc = "3: `11`"]
    SS3 = 3,
}
impl From<SS_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SS_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SS_SEL_A {
    type Ux = u8;
}
impl SS_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SS_SEL_A {
        match self.bits {
            0 => SS_SEL_A::SS0,
            1 => SS_SEL_A::SS1,
            2 => SS_SEL_A::SS2,
            3 => SS_SEL_A::SS3,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_ss0(&self) -> bool {
        *self == SS_SEL_A::SS0
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_ss1(&self) -> bool {
        *self == SS_SEL_A::SS1
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_ss2(&self) -> bool {
        *self == SS_SEL_A::SS2
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_ss3(&self) -> bool {
        *self == SS_SEL_A::SS3
    }
}
#[doc = "Field `ss_sel` writer - "]
pub type SS_SEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SS_SEL_A>;
impl<'a, REG> SS_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ss0(self) -> &'a mut crate::W<REG> {
        self.variant(SS_SEL_A::SS0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn ss1(self) -> &'a mut crate::W<REG> {
        self.variant(SS_SEL_A::SS1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn ss2(self) -> &'a mut crate::W<REG> {
        self.variant(SS_SEL_A::SS2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn ss3(self) -> &'a mut crate::W<REG> {
        self.variant(SS_SEL_A::SS3)
    }
}
#[doc = "Field `ss_owner` reader - "]
pub type SS_OWNER_R = crate::BitReader<SS_OWNER_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SS_OWNER_A {
    #[doc = "0: `0`"]
    SPI_CONTROLLER = 0,
    #[doc = "1: `1`"]
    SOFTWARE = 1,
}
impl From<SS_OWNER_A> for bool {
    #[inline(always)]
    fn from(variant: SS_OWNER_A) -> Self {
        variant as u8 != 0
    }
}
impl SS_OWNER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SS_OWNER_A {
        match self.bits {
            false => SS_OWNER_A::SPI_CONTROLLER,
            true => SS_OWNER_A::SOFTWARE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_spi_controller(&self) -> bool {
        *self == SS_OWNER_A::SPI_CONTROLLER
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == SS_OWNER_A::SOFTWARE
    }
}
#[doc = "Field `ss_owner` writer - "]
pub type SS_OWNER_W<'a, REG> = crate::BitWriter<'a, REG, SS_OWNER_A>;
impl<'a, REG> SS_OWNER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn spi_controller(self) -> &'a mut crate::W<REG> {
        self.variant(SS_OWNER_A::SPI_CONTROLLER)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn software(self) -> &'a mut crate::W<REG> {
        self.variant(SS_OWNER_A::SOFTWARE)
    }
}
#[doc = "Field `ss_level` reader - "]
pub type SS_LEVEL_R = crate::BitReader<SS_LEVEL_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SS_LEVEL_A {
    #[doc = "0: `0`"]
    LOW = 0,
    #[doc = "1: `1`"]
    HIGH = 1,
}
impl From<SS_LEVEL_A> for bool {
    #[inline(always)]
    fn from(variant: SS_LEVEL_A) -> Self {
        variant as u8 != 0
    }
}
impl SS_LEVEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SS_LEVEL_A {
        match self.bits {
            false => SS_LEVEL_A::LOW,
            true => SS_LEVEL_A::HIGH,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SS_LEVEL_A::LOW
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SS_LEVEL_A::HIGH
    }
}
#[doc = "Field `ss_level` writer - "]
pub type SS_LEVEL_W<'a, REG> = crate::BitWriter<'a, REG, SS_LEVEL_A>;
impl<'a, REG> SS_LEVEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(SS_LEVEL_A::LOW)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(SS_LEVEL_A::HIGH)
    }
}
#[doc = "Field `dhb` reader - Discard Hash Burst"]
pub type DHB_R = crate::BitReader<DHB_A>;
#[doc = "Discard Hash Burst\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DHB_A {
    #[doc = "0: Receiving all SPI bursts in the BC period"]
    RECEIVE = 0,
    #[doc = "1: Discard unused SPI bursts"]
    DISCARD = 1,
}
impl From<DHB_A> for bool {
    #[inline(always)]
    fn from(variant: DHB_A) -> Self {
        variant as u8 != 0
    }
}
impl DHB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DHB_A {
        match self.bits {
            false => DHB_A::RECEIVE,
            true => DHB_A::DISCARD,
        }
    }
    #[doc = "Receiving all SPI bursts in the BC period"]
    #[inline(always)]
    pub fn is_receive(&self) -> bool {
        *self == DHB_A::RECEIVE
    }
    #[doc = "Discard unused SPI bursts"]
    #[inline(always)]
    pub fn is_discard(&self) -> bool {
        *self == DHB_A::DISCARD
    }
}
#[doc = "Field `dhb` writer - Discard Hash Burst"]
pub type DHB_W<'a, REG> = crate::BitWriter<'a, REG, DHB_A>;
impl<'a, REG> DHB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receiving all SPI bursts in the BC period"]
    #[inline(always)]
    pub fn receive(self) -> &'a mut crate::W<REG> {
        self.variant(DHB_A::RECEIVE)
    }
    #[doc = "Discard unused SPI bursts"]
    #[inline(always)]
    pub fn discard(self) -> &'a mut crate::W<REG> {
        self.variant(DHB_A::DISCARD)
    }
}
#[doc = "Field `ddb` reader - Dummy Burst Type"]
pub type DDB_R = crate::BitReader<DDB_A>;
#[doc = "Dummy Burst Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DDB_A {
    #[doc = "0: The bit value of dummy SPI burst is zero"]
    ZERO = 0,
    #[doc = "1: The bit value of dummy SPI burst is one"]
    ONE = 1,
}
impl From<DDB_A> for bool {
    #[inline(always)]
    fn from(variant: DDB_A) -> Self {
        variant as u8 != 0
    }
}
impl DDB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DDB_A {
        match self.bits {
            false => DDB_A::ZERO,
            true => DDB_A::ONE,
        }
    }
    #[doc = "The bit value of dummy SPI burst is zero"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == DDB_A::ZERO
    }
    #[doc = "The bit value of dummy SPI burst is one"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == DDB_A::ONE
    }
}
#[doc = "Field `ddb` writer - Dummy Burst Type"]
pub type DDB_W<'a, REG> = crate::BitWriter<'a, REG, DDB_A>;
impl<'a, REG> DDB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The bit value of dummy SPI burst is zero"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(DDB_A::ZERO)
    }
    #[doc = "The bit value of dummy SPI burst is one"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(DDB_A::ONE)
    }
}
#[doc = "Field `rpsm` reader - Rapids Mode Select"]
pub type RPSM_R = crate::BitReader<RPSM_A>;
#[doc = "Rapids Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPSM_A {
    #[doc = "0: Normal write mode"]
    NORMAL = 0,
    #[doc = "1: Rapid write mode"]
    RAPID = 1,
}
impl From<RPSM_A> for bool {
    #[inline(always)]
    fn from(variant: RPSM_A) -> Self {
        variant as u8 != 0
    }
}
impl RPSM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RPSM_A {
        match self.bits {
            false => RPSM_A::NORMAL,
            true => RPSM_A::RAPID,
        }
    }
    #[doc = "Normal write mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == RPSM_A::NORMAL
    }
    #[doc = "Rapid write mode"]
    #[inline(always)]
    pub fn is_rapid(&self) -> bool {
        *self == RPSM_A::RAPID
    }
}
#[doc = "Field `rpsm` writer - Rapids Mode Select"]
pub type RPSM_W<'a, REG> = crate::BitWriter<'a, REG, RPSM_A>;
impl<'a, REG> RPSM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal write mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(RPSM_A::NORMAL)
    }
    #[doc = "Rapid write mode"]
    #[inline(always)]
    pub fn rapid(self) -> &'a mut crate::W<REG> {
        self.variant(RPSM_A::RAPID)
    }
}
#[doc = "Field `sdc` reader - Master Sample Data Control"]
pub type SDC_R = crate::BitReader<SDC_A>;
#[doc = "Master Sample Data Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDC_A {
    #[doc = "0: Normal operation, do not delay the internal read sample point"]
    NORMAL = 0,
    #[doc = "1: Delay the internal read sample point"]
    DELAY = 1,
}
impl From<SDC_A> for bool {
    #[inline(always)]
    fn from(variant: SDC_A) -> Self {
        variant as u8 != 0
    }
}
impl SDC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDC_A {
        match self.bits {
            false => SDC_A::NORMAL,
            true => SDC_A::DELAY,
        }
    }
    #[doc = "Normal operation, do not delay the internal read sample point"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SDC_A::NORMAL
    }
    #[doc = "Delay the internal read sample point"]
    #[inline(always)]
    pub fn is_delay(&self) -> bool {
        *self == SDC_A::DELAY
    }
}
#[doc = "Field `sdc` writer - Master Sample Data Control"]
pub type SDC_W<'a, REG> = crate::BitWriter<'a, REG, SDC_A>;
impl<'a, REG> SDC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation, do not delay the internal read sample point"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(SDC_A::NORMAL)
    }
    #[doc = "Delay the internal read sample point"]
    #[inline(always)]
    pub fn delay(self) -> &'a mut crate::W<REG> {
        self.variant(SDC_A::DELAY)
    }
}
#[doc = "Field `fbs` reader - First Transmit Bit Select"]
pub type FBS_R = crate::BitReader<FBS_A>;
#[doc = "First Transmit Bit Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FBS_A {
    #[doc = "0: MSB first"]
    MSB = 0,
    #[doc = "1: LSB first"]
    LSB = 1,
}
impl From<FBS_A> for bool {
    #[inline(always)]
    fn from(variant: FBS_A) -> Self {
        variant as u8 != 0
    }
}
impl FBS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FBS_A {
        match self.bits {
            false => FBS_A::MSB,
            true => FBS_A::LSB,
        }
    }
    #[doc = "MSB first"]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == FBS_A::MSB
    }
    #[doc = "LSB first"]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == FBS_A::LSB
    }
}
#[doc = "Field `fbs` writer - First Transmit Bit Select"]
pub type FBS_W<'a, REG> = crate::BitWriter<'a, REG, FBS_A>;
impl<'a, REG> FBS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MSB first"]
    #[inline(always)]
    pub fn msb(self) -> &'a mut crate::W<REG> {
        self.variant(FBS_A::MSB)
    }
    #[doc = "LSB first"]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut crate::W<REG> {
        self.variant(FBS_A::LSB)
    }
}
#[doc = "Field `sdm` reader - Master Sample Data Mode"]
pub type SDM_R = crate::BitReader<SDM_A>;
#[doc = "Master Sample Data Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDM_A {
    #[doc = "0: delay sample mode"]
    DELAY = 0,
    #[doc = "1: normal sample mode"]
    NORMAL = 1,
}
impl From<SDM_A> for bool {
    #[inline(always)]
    fn from(variant: SDM_A) -> Self {
        variant as u8 != 0
    }
}
impl SDM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDM_A {
        match self.bits {
            false => SDM_A::DELAY,
            true => SDM_A::NORMAL,
        }
    }
    #[doc = "delay sample mode"]
    #[inline(always)]
    pub fn is_delay(&self) -> bool {
        *self == SDM_A::DELAY
    }
    #[doc = "normal sample mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SDM_A::NORMAL
    }
}
#[doc = "Field `sdm` writer - Master Sample Data Mode"]
pub type SDM_W<'a, REG> = crate::BitWriter<'a, REG, SDM_A>;
impl<'a, REG> SDM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "delay sample mode"]
    #[inline(always)]
    pub fn delay(self) -> &'a mut crate::W<REG> {
        self.variant(SDM_A::DELAY)
    }
    #[doc = "normal sample mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(SDM_A::NORMAL)
    }
}
#[doc = "Field `sddm` reader - Sending Data Delay Mode"]
pub type SDDM_R = crate::BitReader<SDDM_A>;
#[doc = "Sending Data Delay Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDDM_A {
    #[doc = "0: normal sending"]
    NORMAL = 0,
    #[doc = "1: delay sending"]
    DELAY = 1,
}
impl From<SDDM_A> for bool {
    #[inline(always)]
    fn from(variant: SDDM_A) -> Self {
        variant as u8 != 0
    }
}
impl SDDM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDDM_A {
        match self.bits {
            false => SDDM_A::NORMAL,
            true => SDDM_A::DELAY,
        }
    }
    #[doc = "normal sending"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SDDM_A::NORMAL
    }
    #[doc = "delay sending"]
    #[inline(always)]
    pub fn is_delay(&self) -> bool {
        *self == SDDM_A::DELAY
    }
}
#[doc = "Field `sddm` writer - Sending Data Delay Mode"]
pub type SDDM_W<'a, REG> = crate::BitWriter<'a, REG, SDDM_A>;
impl<'a, REG> SDDM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal sending"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(SDDM_A::NORMAL)
    }
    #[doc = "delay sending"]
    #[inline(always)]
    pub fn delay(self) -> &'a mut crate::W<REG> {
        self.variant(SDDM_A::DELAY)
    }
}
#[doc = "Field `sdc1` reader - Master Sample Data Control register1"]
pub type SDC1_R = crate::BitReader<SDC1_A>;
#[doc = "Master Sample Data Control register1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDC1_A {
    #[doc = "0: normal operation, do not delay the internal read sample point"]
    NORMAL = 0,
    #[doc = "1: delay the internal read sample point"]
    DELAY = 1,
}
impl From<SDC1_A> for bool {
    #[inline(always)]
    fn from(variant: SDC1_A) -> Self {
        variant as u8 != 0
    }
}
impl SDC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDC1_A {
        match self.bits {
            false => SDC1_A::NORMAL,
            true => SDC1_A::DELAY,
        }
    }
    #[doc = "normal operation, do not delay the internal read sample point"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SDC1_A::NORMAL
    }
    #[doc = "delay the internal read sample point"]
    #[inline(always)]
    pub fn is_delay(&self) -> bool {
        *self == SDC1_A::DELAY
    }
}
#[doc = "Field `sdc1` writer - Master Sample Data Control register1"]
pub type SDC1_W<'a, REG> = crate::BitWriter<'a, REG, SDC1_A>;
impl<'a, REG> SDC1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal operation, do not delay the internal read sample point"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(SDC1_A::NORMAL)
    }
    #[doc = "delay the internal read sample point"]
    #[inline(always)]
    pub fn delay(self) -> &'a mut crate::W<REG> {
        self.variant(SDC1_A::DELAY)
    }
}
#[doc = "Field `xch` reader - Exchange Burst"]
pub type XCH_R = crate::BitReader<XCH_A>;
#[doc = "Exchange Burst\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XCH_A {
    #[doc = "0: `0`"]
    IDLE = 0,
    #[doc = "1: `1`"]
    INITIATE_EXCHANGE = 1,
}
impl From<XCH_A> for bool {
    #[inline(always)]
    fn from(variant: XCH_A) -> Self {
        variant as u8 != 0
    }
}
impl XCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> XCH_A {
        match self.bits {
            false => XCH_A::IDLE,
            true => XCH_A::INITIATE_EXCHANGE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == XCH_A::IDLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_initiate_exchange(&self) -> bool {
        *self == XCH_A::INITIATE_EXCHANGE
    }
}
#[doc = "Field `xch` writer - Exchange Burst"]
pub type XCH_W<'a, REG> = crate::BitWriter<'a, REG, XCH_A>;
impl<'a, REG> XCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(XCH_A::IDLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn initiate_exchange(self) -> &'a mut crate::W<REG> {
        self.variant(XCH_A::INITIATE_EXCHANGE)
    }
}
impl R {
    #[doc = "Bit 0 - SPI Clock/Data Phase Control"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI Clock Polarity Control"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SPI Chip Select Signal Polarity Control"]
    #[inline(always)]
    pub fn spol(&self) -> SPOL_R {
        SPOL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ssctl(&self) -> SSCTL_R {
        SSCTL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn ss_sel(&self) -> SS_SEL_R {
        SS_SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ss_owner(&self) -> SS_OWNER_R {
        SS_OWNER_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ss_level(&self) -> SS_LEVEL_R {
        SS_LEVEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Discard Hash Burst"]
    #[inline(always)]
    pub fn dhb(&self) -> DHB_R {
        DHB_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Dummy Burst Type"]
    #[inline(always)]
    pub fn ddb(&self) -> DDB_R {
        DDB_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Rapids Mode Select"]
    #[inline(always)]
    pub fn rpsm(&self) -> RPSM_R {
        RPSM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Master Sample Data Control"]
    #[inline(always)]
    pub fn sdc(&self) -> SDC_R {
        SDC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - First Transmit Bit Select"]
    #[inline(always)]
    pub fn fbs(&self) -> FBS_R {
        FBS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Master Sample Data Mode"]
    #[inline(always)]
    pub fn sdm(&self) -> SDM_R {
        SDM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Sending Data Delay Mode"]
    #[inline(always)]
    pub fn sddm(&self) -> SDDM_R {
        SDDM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Master Sample Data Control register1"]
    #[inline(always)]
    pub fn sdc1(&self) -> SDC1_R {
        SDC1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - Exchange Burst"]
    #[inline(always)]
    pub fn xch(&self) -> XCH_R {
        XCH_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI Clock/Data Phase Control"]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<SPI_TCR_SPEC> {
        CPHA_W::new(self, 0)
    }
    #[doc = "Bit 1 - SPI Clock Polarity Control"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<SPI_TCR_SPEC> {
        CPOL_W::new(self, 1)
    }
    #[doc = "Bit 2 - SPI Chip Select Signal Polarity Control"]
    #[inline(always)]
    #[must_use]
    pub fn spol(&mut self) -> SPOL_W<SPI_TCR_SPEC> {
        SPOL_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn ssctl(&mut self) -> SSCTL_W<SPI_TCR_SPEC> {
        SSCTL_W::new(self, 3)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn ss_sel(&mut self) -> SS_SEL_W<SPI_TCR_SPEC> {
        SS_SEL_W::new(self, 4)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn ss_owner(&mut self) -> SS_OWNER_W<SPI_TCR_SPEC> {
        SS_OWNER_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn ss_level(&mut self) -> SS_LEVEL_W<SPI_TCR_SPEC> {
        SS_LEVEL_W::new(self, 7)
    }
    #[doc = "Bit 8 - Discard Hash Burst"]
    #[inline(always)]
    #[must_use]
    pub fn dhb(&mut self) -> DHB_W<SPI_TCR_SPEC> {
        DHB_W::new(self, 8)
    }
    #[doc = "Bit 9 - Dummy Burst Type"]
    #[inline(always)]
    #[must_use]
    pub fn ddb(&mut self) -> DDB_W<SPI_TCR_SPEC> {
        DDB_W::new(self, 9)
    }
    #[doc = "Bit 10 - Rapids Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn rpsm(&mut self) -> RPSM_W<SPI_TCR_SPEC> {
        RPSM_W::new(self, 10)
    }
    #[doc = "Bit 11 - Master Sample Data Control"]
    #[inline(always)]
    #[must_use]
    pub fn sdc(&mut self) -> SDC_W<SPI_TCR_SPEC> {
        SDC_W::new(self, 11)
    }
    #[doc = "Bit 12 - First Transmit Bit Select"]
    #[inline(always)]
    #[must_use]
    pub fn fbs(&mut self) -> FBS_W<SPI_TCR_SPEC> {
        FBS_W::new(self, 12)
    }
    #[doc = "Bit 13 - Master Sample Data Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sdm(&mut self) -> SDM_W<SPI_TCR_SPEC> {
        SDM_W::new(self, 13)
    }
    #[doc = "Bit 14 - Sending Data Delay Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sddm(&mut self) -> SDDM_W<SPI_TCR_SPEC> {
        SDDM_W::new(self, 14)
    }
    #[doc = "Bit 15 - Master Sample Data Control register1"]
    #[inline(always)]
    #[must_use]
    pub fn sdc1(&mut self) -> SDC1_W<SPI_TCR_SPEC> {
        SDC1_W::new(self, 15)
    }
    #[doc = "Bit 31 - Exchange Burst"]
    #[inline(always)]
    #[must_use]
    pub fn xch(&mut self) -> XCH_W<SPI_TCR_SPEC> {
        XCH_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SPI Transfer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_tcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_tcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_TCR_SPEC;
impl crate::RegisterSpec for SPI_TCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_tcr::R`](R) reader structure"]
impl crate::Readable for SPI_TCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_tcr::W`](W) writer structure"]
impl crate::Writable for SPI_TCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets spi_tcr to value 0"]
impl crate::Resettable for SPI_TCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
