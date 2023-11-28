#[doc = "Register `hmic_sts` reader"]
pub type R = crate::R<HMIC_STS_SPEC>;
#[doc = "Register `hmic_sts` writer"]
pub type W = crate::W<HMIC_STS_SPEC>;
#[doc = "Field `mic_det_st` reader - MIC detect pending interrupt"]
pub type MIC_DET_ST_R = crate::BitReader<MIC_DET_ST_A>;
#[doc = "MIC detect pending interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIC_DET_ST_A {
    #[doc = "0: No pending IRQ"]
    NO_PENDING = 0,
    #[doc = "1: Pending IRQ"]
    PENDING = 1,
}
impl From<MIC_DET_ST_A> for bool {
    #[inline(always)]
    fn from(variant: MIC_DET_ST_A) -> Self {
        variant as u8 != 0
    }
}
impl MIC_DET_ST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MIC_DET_ST_A {
        match self.bits {
            false => MIC_DET_ST_A::NO_PENDING,
            true => MIC_DET_ST_A::PENDING,
        }
    }
    #[doc = "No pending IRQ"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == MIC_DET_ST_A::NO_PENDING
    }
    #[doc = "Pending IRQ"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == MIC_DET_ST_A::PENDING
    }
}
#[doc = "Field `mic_det_st` writer - MIC detect pending interrupt"]
pub type MIC_DET_ST_W<'a, REG> = crate::BitWriter1C<'a, REG, MIC_DET_ST_A>;
impl<'a, REG> MIC_DET_ST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No pending IRQ"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut crate::W<REG> {
        self.variant(MIC_DET_ST_A::NO_PENDING)
    }
    #[doc = "Pending IRQ"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(MIC_DET_ST_A::PENDING)
    }
}
#[doc = "Field `jack_det_iirq` reader - Jack input detect pending interrupt"]
pub type JACK_DET_IIRQ_R = crate::BitReader<JACK_DET_IIRQ_A>;
#[doc = "Jack input detect pending interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JACK_DET_IIRQ_A {
    #[doc = "0: No Pending IRQ"]
    NO_PENDING = 0,
    #[doc = "1: Pending IRQ"]
    PENDING = 1,
}
impl From<JACK_DET_IIRQ_A> for bool {
    #[inline(always)]
    fn from(variant: JACK_DET_IIRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl JACK_DET_IIRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JACK_DET_IIRQ_A {
        match self.bits {
            false => JACK_DET_IIRQ_A::NO_PENDING,
            true => JACK_DET_IIRQ_A::PENDING,
        }
    }
    #[doc = "No Pending IRQ"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == JACK_DET_IIRQ_A::NO_PENDING
    }
    #[doc = "Pending IRQ"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == JACK_DET_IIRQ_A::PENDING
    }
}
#[doc = "Field `jack_det_iirq` writer - Jack input detect pending interrupt"]
pub type JACK_DET_IIRQ_W<'a, REG> = crate::BitWriter1C<'a, REG, JACK_DET_IIRQ_A>;
impl<'a, REG> JACK_DET_IIRQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Pending IRQ"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut crate::W<REG> {
        self.variant(JACK_DET_IIRQ_A::NO_PENDING)
    }
    #[doc = "Pending IRQ"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(JACK_DET_IIRQ_A::PENDING)
    }
}
#[doc = "Field `jack_det_oirq` reader - Jack output detect pending interrupt"]
pub type JACK_DET_OIRQ_R = crate::BitReader<JACK_DET_OIRQ_A>;
#[doc = "Jack output detect pending interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JACK_DET_OIRQ_A {
    #[doc = "0: No Pending IRQ"]
    NO_PENDING = 0,
    #[doc = "1: Pending IRQ"]
    PENDING = 1,
}
impl From<JACK_DET_OIRQ_A> for bool {
    #[inline(always)]
    fn from(variant: JACK_DET_OIRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl JACK_DET_OIRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JACK_DET_OIRQ_A {
        match self.bits {
            false => JACK_DET_OIRQ_A::NO_PENDING,
            true => JACK_DET_OIRQ_A::PENDING,
        }
    }
    #[doc = "No Pending IRQ"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == JACK_DET_OIRQ_A::NO_PENDING
    }
    #[doc = "Pending IRQ"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == JACK_DET_OIRQ_A::PENDING
    }
}
#[doc = "Field `jack_det_oirq` writer - Jack output detect pending interrupt"]
pub type JACK_DET_OIRQ_W<'a, REG> = crate::BitWriter1C<'a, REG, JACK_DET_OIRQ_A>;
impl<'a, REG> JACK_DET_OIRQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Pending IRQ"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut crate::W<REG> {
        self.variant(JACK_DET_OIRQ_A::NO_PENDING)
    }
    #[doc = "Pending IRQ"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(JACK_DET_OIRQ_A::PENDING)
    }
}
#[doc = "Field `hmic_data` reader - HMIC Average Data"]
pub type HMIC_DATA_R = crate::FieldReader;
#[doc = "Field `mdata_discard` reader - After MIC DATA data is received, the first N-data will be discarded."]
pub type MDATA_DISCARD_R = crate::FieldReader<MDATA_DISCARD_A>;
#[doc = "After MIC DATA data is received, the first N-data will be discarded.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MDATA_DISCARD_A {
    #[doc = "0: None discarded"]
    NONE = 0,
    #[doc = "1: 1-data discarded"]
    _1_DATA = 1,
    #[doc = "2: 2-data discarded"]
    _2_DATA = 2,
    #[doc = "3: 4-data discarded"]
    _4_DATA = 3,
}
impl From<MDATA_DISCARD_A> for u8 {
    #[inline(always)]
    fn from(variant: MDATA_DISCARD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MDATA_DISCARD_A {
    type Ux = u8;
}
impl MDATA_DISCARD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MDATA_DISCARD_A {
        match self.bits {
            0 => MDATA_DISCARD_A::NONE,
            1 => MDATA_DISCARD_A::_1_DATA,
            2 => MDATA_DISCARD_A::_2_DATA,
            3 => MDATA_DISCARD_A::_4_DATA,
            _ => unreachable!(),
        }
    }
    #[doc = "None discarded"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == MDATA_DISCARD_A::NONE
    }
    #[doc = "1-data discarded"]
    #[inline(always)]
    pub fn is_1_data(&self) -> bool {
        *self == MDATA_DISCARD_A::_1_DATA
    }
    #[doc = "2-data discarded"]
    #[inline(always)]
    pub fn is_2_data(&self) -> bool {
        *self == MDATA_DISCARD_A::_2_DATA
    }
    #[doc = "4-data discarded"]
    #[inline(always)]
    pub fn is_4_data(&self) -> bool {
        *self == MDATA_DISCARD_A::_4_DATA
    }
}
#[doc = "Field `mdata_discard` writer - After MIC DATA data is received, the first N-data will be discarded."]
pub type MDATA_DISCARD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, MDATA_DISCARD_A>;
impl<'a, REG> MDATA_DISCARD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None discarded"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(MDATA_DISCARD_A::NONE)
    }
    #[doc = "1-data discarded"]
    #[inline(always)]
    pub fn _1_data(self) -> &'a mut crate::W<REG> {
        self.variant(MDATA_DISCARD_A::_1_DATA)
    }
    #[doc = "2-data discarded"]
    #[inline(always)]
    pub fn _2_data(self) -> &'a mut crate::W<REG> {
        self.variant(MDATA_DISCARD_A::_2_DATA)
    }
    #[doc = "4-data discarded"]
    #[inline(always)]
    pub fn _4_data(self) -> &'a mut crate::W<REG> {
        self.variant(MDATA_DISCARD_A::_4_DATA)
    }
}
impl R {
    #[doc = "Bit 0 - MIC detect pending interrupt"]
    #[inline(always)]
    pub fn mic_det_st(&self) -> MIC_DET_ST_R {
        MIC_DET_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Jack input detect pending interrupt"]
    #[inline(always)]
    pub fn jack_det_iirq(&self) -> JACK_DET_IIRQ_R {
        JACK_DET_IIRQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Jack output detect pending interrupt"]
    #[inline(always)]
    pub fn jack_det_oirq(&self) -> JACK_DET_OIRQ_R {
        JACK_DET_OIRQ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:12 - HMIC Average Data"]
    #[inline(always)]
    pub fn hmic_data(&self) -> HMIC_DATA_R {
        HMIC_DATA_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:14 - After MIC DATA data is received, the first N-data will be discarded."]
    #[inline(always)]
    pub fn mdata_discard(&self) -> MDATA_DISCARD_R {
        MDATA_DISCARD_R::new(((self.bits >> 13) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - MIC detect pending interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn mic_det_st(&mut self) -> MIC_DET_ST_W<HMIC_STS_SPEC> {
        MIC_DET_ST_W::new(self, 0)
    }
    #[doc = "Bit 3 - Jack input detect pending interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn jack_det_iirq(&mut self) -> JACK_DET_IIRQ_W<HMIC_STS_SPEC> {
        JACK_DET_IIRQ_W::new(self, 3)
    }
    #[doc = "Bit 4 - Jack output detect pending interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn jack_det_oirq(&mut self) -> JACK_DET_OIRQ_W<HMIC_STS_SPEC> {
        JACK_DET_OIRQ_W::new(self, 4)
    }
    #[doc = "Bits 13:14 - After MIC DATA data is received, the first N-data will be discarded."]
    #[inline(always)]
    #[must_use]
    pub fn mdata_discard(&mut self) -> MDATA_DISCARD_W<HMIC_STS_SPEC> {
        MDATA_DISCARD_W::new(self, 13)
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
#[doc = "HMIC Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hmic_sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hmic_sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HMIC_STS_SPEC;
impl crate::RegisterSpec for HMIC_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hmic_sts::R`](R) reader structure"]
impl crate::Readable for HMIC_STS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hmic_sts::W`](W) writer structure"]
impl crate::Writable for HMIC_STS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x19;
}
#[doc = "`reset()` method sets hmic_sts to value 0x6000"]
impl crate::Resettable for HMIC_STS_SPEC {
    const RESET_VALUE: Self::Ux = 0x6000;
}
