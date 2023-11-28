#[doc = "Register `i2s_pcm_fmt1` reader"]
pub type R = crate::R<I2S_PCM_FMT1_SPEC>;
#[doc = "Register `i2s_pcm_fmt1` writer"]
pub type W = crate::W<I2S_PCM_FMT1_SPEC>;
#[doc = "Field `tx_pdm` reader - Tx PCM Data Mode"]
pub type TX_PDM_R = crate::FieldReader<TX_PDM_A>;
#[doc = "Tx PCM Data Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TX_PDM_A {
    #[doc = "0: Linear PCM"]
    LINEAR = 0,
    #[doc = "2: 8-bit u-law"]
    U_LAW = 2,
    #[doc = "3: 8-bit A-law"]
    A_LAW = 3,
}
impl From<TX_PDM_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_PDM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TX_PDM_A {
    type Ux = u8;
}
impl TX_PDM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TX_PDM_A {
        match self.bits {
            0 => TX_PDM_A::LINEAR,
            2 => TX_PDM_A::U_LAW,
            3 => TX_PDM_A::A_LAW,
            _ => unreachable!(),
        }
    }
    #[doc = "Linear PCM"]
    #[inline(always)]
    pub fn is_linear(&self) -> bool {
        *self == TX_PDM_A::LINEAR
    }
    #[doc = "8-bit u-law"]
    #[inline(always)]
    pub fn is_u_law(&self) -> bool {
        *self == TX_PDM_A::U_LAW
    }
    #[doc = "8-bit A-law"]
    #[inline(always)]
    pub fn is_a_law(&self) -> bool {
        *self == TX_PDM_A::A_LAW
    }
}
#[doc = "Field `tx_pdm` writer - Tx PCM Data Mode"]
pub type TX_PDM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TX_PDM_A>;
impl<'a, REG> TX_PDM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Linear PCM"]
    #[inline(always)]
    pub fn linear(self) -> &'a mut crate::W<REG> {
        self.variant(TX_PDM_A::LINEAR)
    }
    #[doc = "8-bit u-law"]
    #[inline(always)]
    pub fn u_law(self) -> &'a mut crate::W<REG> {
        self.variant(TX_PDM_A::U_LAW)
    }
    #[doc = "8-bit A-law"]
    #[inline(always)]
    pub fn a_law(self) -> &'a mut crate::W<REG> {
        self.variant(TX_PDM_A::A_LAW)
    }
}
#[doc = "Field `rx_pdm` reader - Rx PCM Data Mode"]
pub type RX_PDM_R = crate::FieldReader<RX_PDM_A>;
#[doc = "Rx PCM Data Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RX_PDM_A {
    #[doc = "0: Linear PCM"]
    LINEAR = 0,
    #[doc = "2: 8-bit u-law"]
    U_LAW = 2,
    #[doc = "3: 8-bit A-law"]
    A_LAW = 3,
}
impl From<RX_PDM_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_PDM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RX_PDM_A {
    type Ux = u8;
}
impl RX_PDM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RX_PDM_A {
        match self.bits {
            0 => RX_PDM_A::LINEAR,
            2 => RX_PDM_A::U_LAW,
            3 => RX_PDM_A::A_LAW,
            _ => unreachable!(),
        }
    }
    #[doc = "Linear PCM"]
    #[inline(always)]
    pub fn is_linear(&self) -> bool {
        *self == RX_PDM_A::LINEAR
    }
    #[doc = "8-bit u-law"]
    #[inline(always)]
    pub fn is_u_law(&self) -> bool {
        *self == RX_PDM_A::U_LAW
    }
    #[doc = "8-bit A-law"]
    #[inline(always)]
    pub fn is_a_law(&self) -> bool {
        *self == RX_PDM_A::A_LAW
    }
}
#[doc = "Field `rx_pdm` writer - Rx PCM Data Mode"]
pub type RX_PDM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RX_PDM_A>;
impl<'a, REG> RX_PDM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Linear PCM"]
    #[inline(always)]
    pub fn linear(self) -> &'a mut crate::W<REG> {
        self.variant(RX_PDM_A::LINEAR)
    }
    #[doc = "8-bit u-law"]
    #[inline(always)]
    pub fn u_law(self) -> &'a mut crate::W<REG> {
        self.variant(RX_PDM_A::U_LAW)
    }
    #[doc = "8-bit A-law"]
    #[inline(always)]
    pub fn a_law(self) -> &'a mut crate::W<REG> {
        self.variant(RX_PDM_A::A_LAW)
    }
}
#[doc = "Field `sext` reader - Sign Extended in Slot (Sample Resolution &lt; Slot Width)"]
pub type SEXT_R = crate::FieldReader<SEXT_A>;
#[doc = "Sign Extended in Slot (Sample Resolution &lt; Slot Width)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEXT_A {
    #[doc = "0: Zeros or audio gain padding at LSB position"]
    ZERO = 0,
    #[doc = "1: Sign extension at MSB position"]
    SIGN = 1,
    #[doc = "3: Transfer 0 after each sample in each slot"]
    TRANSFER0 = 3,
}
impl From<SEXT_A> for u8 {
    #[inline(always)]
    fn from(variant: SEXT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SEXT_A {
    type Ux = u8;
}
impl SEXT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SEXT_A {
        match self.bits {
            0 => SEXT_A::ZERO,
            1 => SEXT_A::SIGN,
            3 => SEXT_A::TRANSFER0,
            _ => unreachable!(),
        }
    }
    #[doc = "Zeros or audio gain padding at LSB position"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == SEXT_A::ZERO
    }
    #[doc = "Sign extension at MSB position"]
    #[inline(always)]
    pub fn is_sign(&self) -> bool {
        *self == SEXT_A::SIGN
    }
    #[doc = "Transfer 0 after each sample in each slot"]
    #[inline(always)]
    pub fn is_transfer0(&self) -> bool {
        *self == SEXT_A::TRANSFER0
    }
}
#[doc = "Field `sext` writer - Sign Extended in Slot (Sample Resolution &lt; Slot Width)"]
pub type SEXT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SEXT_A>;
impl<'a, REG> SEXT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Zeros or audio gain padding at LSB position"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(SEXT_A::ZERO)
    }
    #[doc = "Sign extension at MSB position"]
    #[inline(always)]
    pub fn sign(self) -> &'a mut crate::W<REG> {
        self.variant(SEXT_A::SIGN)
    }
    #[doc = "Transfer 0 after each sample in each slot"]
    #[inline(always)]
    pub fn transfer0(self) -> &'a mut crate::W<REG> {
        self.variant(SEXT_A::TRANSFER0)
    }
}
#[doc = "Field `tx_mls` reader - Tx MSB/LSB First Select"]
pub type TX_MLS_R = crate::BitReader<TX_MLS_A>;
#[doc = "Tx MSB/LSB First Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_MLS_A {
    #[doc = "0: `0`"]
    MSB = 0,
    #[doc = "1: `1`"]
    LSB = 1,
}
impl From<TX_MLS_A> for bool {
    #[inline(always)]
    fn from(variant: TX_MLS_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_MLS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TX_MLS_A {
        match self.bits {
            false => TX_MLS_A::MSB,
            true => TX_MLS_A::LSB,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == TX_MLS_A::MSB
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == TX_MLS_A::LSB
    }
}
#[doc = "Field `tx_mls` writer - Tx MSB/LSB First Select"]
pub type TX_MLS_W<'a, REG> = crate::BitWriter<'a, REG, TX_MLS_A>;
impl<'a, REG> TX_MLS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn msb(self) -> &'a mut crate::W<REG> {
        self.variant(TX_MLS_A::MSB)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut crate::W<REG> {
        self.variant(TX_MLS_A::LSB)
    }
}
#[doc = "Field `rx_mls` reader - Rx MSB/LSB First Select"]
pub type RX_MLS_R = crate::BitReader<RX_MLS_A>;
#[doc = "Rx MSB/LSB First Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_MLS_A {
    #[doc = "0: `0`"]
    MSB = 0,
    #[doc = "1: `1`"]
    LSB = 1,
}
impl From<RX_MLS_A> for bool {
    #[inline(always)]
    fn from(variant: RX_MLS_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_MLS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RX_MLS_A {
        match self.bits {
            false => RX_MLS_A::MSB,
            true => RX_MLS_A::LSB,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == RX_MLS_A::MSB
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == RX_MLS_A::LSB
    }
}
#[doc = "Field `rx_mls` writer - Rx MSB/LSB First Select"]
pub type RX_MLS_W<'a, REG> = crate::BitWriter<'a, REG, RX_MLS_A>;
impl<'a, REG> RX_MLS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn msb(self) -> &'a mut crate::W<REG> {
        self.variant(RX_MLS_A::MSB)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut crate::W<REG> {
        self.variant(RX_MLS_A::LSB)
    }
}
impl R {
    #[doc = "Bits 0:1 - Tx PCM Data Mode"]
    #[inline(always)]
    pub fn tx_pdm(&self) -> TX_PDM_R {
        TX_PDM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Rx PCM Data Mode"]
    #[inline(always)]
    pub fn rx_pdm(&self) -> RX_PDM_R {
        RX_PDM_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Sign Extended in Slot (Sample Resolution &lt; Slot Width)"]
    #[inline(always)]
    pub fn sext(&self) -> SEXT_R {
        SEXT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Tx MSB/LSB First Select"]
    #[inline(always)]
    pub fn tx_mls(&self) -> TX_MLS_R {
        TX_MLS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rx MSB/LSB First Select"]
    #[inline(always)]
    pub fn rx_mls(&self) -> RX_MLS_R {
        RX_MLS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Tx PCM Data Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm(&mut self) -> TX_PDM_W<I2S_PCM_FMT1_SPEC> {
        TX_PDM_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Rx PCM Data Mode"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pdm(&mut self) -> RX_PDM_W<I2S_PCM_FMT1_SPEC> {
        RX_PDM_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Sign Extended in Slot (Sample Resolution &lt; Slot Width)"]
    #[inline(always)]
    #[must_use]
    pub fn sext(&mut self) -> SEXT_W<I2S_PCM_FMT1_SPEC> {
        SEXT_W::new(self, 4)
    }
    #[doc = "Bit 6 - Tx MSB/LSB First Select"]
    #[inline(always)]
    #[must_use]
    pub fn tx_mls(&mut self) -> TX_MLS_W<I2S_PCM_FMT1_SPEC> {
        TX_MLS_W::new(self, 6)
    }
    #[doc = "Bit 7 - Rx MSB/LSB First Select"]
    #[inline(always)]
    #[must_use]
    pub fn rx_mls(&mut self) -> RX_MLS_W<I2S_PCM_FMT1_SPEC> {
        RX_MLS_W::new(self, 7)
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
#[doc = "I2S/PCM Format Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_fmt1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_fmt1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2S_PCM_FMT1_SPEC;
impl crate::RegisterSpec for I2S_PCM_FMT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_pcm_fmt1::R`](R) reader structure"]
impl crate::Readable for I2S_PCM_FMT1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2s_pcm_fmt1::W`](W) writer structure"]
impl crate::Writable for I2S_PCM_FMT1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets i2s_pcm_fmt1 to value 0"]
impl crate::Resettable for I2S_PCM_FMT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
