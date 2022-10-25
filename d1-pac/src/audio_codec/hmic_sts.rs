#[doc = "Register `hmic_sts` reader"]
pub struct R(crate::R<HMIC_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HMIC_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HMIC_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HMIC_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `hmic_sts` writer"]
pub struct W(crate::W<HMIC_STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HMIC_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<HMIC_STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HMIC_STS_SPEC>) -> Self {
        W(writer)
    }
}
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
    pub fn variant(&self) -> MIC_DET_ST_A {
        match self.bits {
            false => MIC_DET_ST_A::NO_PENDING,
            true => MIC_DET_ST_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == MIC_DET_ST_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == MIC_DET_ST_A::PENDING
    }
}
#[doc = "Field `mic_det_st` writer - MIC detect pending interrupt"]
pub type MIC_DET_ST_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, HMIC_STS_SPEC, MIC_DET_ST_A, O>;
impl<'a, const O: u8> MIC_DET_ST_W<'a, O> {
    #[doc = "No pending IRQ"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(MIC_DET_ST_A::NO_PENDING)
    }
    #[doc = "Pending IRQ"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
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
    pub fn variant(&self) -> JACK_DET_IIRQ_A {
        match self.bits {
            false => JACK_DET_IIRQ_A::NO_PENDING,
            true => JACK_DET_IIRQ_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == JACK_DET_IIRQ_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == JACK_DET_IIRQ_A::PENDING
    }
}
#[doc = "Field `jack_det_iirq` writer - Jack input detect pending interrupt"]
pub type JACK_DET_IIRQ_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, HMIC_STS_SPEC, JACK_DET_IIRQ_A, O>;
impl<'a, const O: u8> JACK_DET_IIRQ_W<'a, O> {
    #[doc = "No Pending IRQ"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(JACK_DET_IIRQ_A::NO_PENDING)
    }
    #[doc = "Pending IRQ"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
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
    pub fn variant(&self) -> JACK_DET_OIRQ_A {
        match self.bits {
            false => JACK_DET_OIRQ_A::NO_PENDING,
            true => JACK_DET_OIRQ_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == JACK_DET_OIRQ_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == JACK_DET_OIRQ_A::PENDING
    }
}
#[doc = "Field `jack_det_oirq` writer - Jack output detect pending interrupt"]
pub type JACK_DET_OIRQ_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, HMIC_STS_SPEC, JACK_DET_OIRQ_A, O>;
impl<'a, const O: u8> JACK_DET_OIRQ_W<'a, O> {
    #[doc = "No Pending IRQ"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(JACK_DET_OIRQ_A::NO_PENDING)
    }
    #[doc = "Pending IRQ"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(JACK_DET_OIRQ_A::PENDING)
    }
}
#[doc = "Field `hmic_data` reader - HMIC Average Data"]
pub type HMIC_DATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `mdata_discard` reader - After MIC DATA data is received, the first N-data will be discarded."]
pub type MDATA_DISCARD_R = crate::FieldReader<u8, MDATA_DISCARD_A>;
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
impl MDATA_DISCARD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDATA_DISCARD_A {
        match self.bits {
            0 => MDATA_DISCARD_A::NONE,
            1 => MDATA_DISCARD_A::_1_DATA,
            2 => MDATA_DISCARD_A::_2_DATA,
            3 => MDATA_DISCARD_A::_4_DATA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == MDATA_DISCARD_A::NONE
    }
    #[doc = "Checks if the value of the field is `_1_DATA`"]
    #[inline(always)]
    pub fn is_1_data(&self) -> bool {
        *self == MDATA_DISCARD_A::_1_DATA
    }
    #[doc = "Checks if the value of the field is `_2_DATA`"]
    #[inline(always)]
    pub fn is_2_data(&self) -> bool {
        *self == MDATA_DISCARD_A::_2_DATA
    }
    #[doc = "Checks if the value of the field is `_4_DATA`"]
    #[inline(always)]
    pub fn is_4_data(&self) -> bool {
        *self == MDATA_DISCARD_A::_4_DATA
    }
}
#[doc = "Field `mdata_discard` writer - After MIC DATA data is received, the first N-data will be discarded."]
pub type MDATA_DISCARD_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, HMIC_STS_SPEC, u8, MDATA_DISCARD_A, 2, O>;
impl<'a, const O: u8> MDATA_DISCARD_W<'a, O> {
    #[doc = "None discarded"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(MDATA_DISCARD_A::NONE)
    }
    #[doc = "1-data discarded"]
    #[inline(always)]
    pub fn _1_data(self) -> &'a mut W {
        self.variant(MDATA_DISCARD_A::_1_DATA)
    }
    #[doc = "2-data discarded"]
    #[inline(always)]
    pub fn _2_data(self) -> &'a mut W {
        self.variant(MDATA_DISCARD_A::_2_DATA)
    }
    #[doc = "4-data discarded"]
    #[inline(always)]
    pub fn _4_data(self) -> &'a mut W {
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
    pub fn mic_det_st(&mut self) -> MIC_DET_ST_W<0> {
        MIC_DET_ST_W::new(self)
    }
    #[doc = "Bit 3 - Jack input detect pending interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn jack_det_iirq(&mut self) -> JACK_DET_IIRQ_W<3> {
        JACK_DET_IIRQ_W::new(self)
    }
    #[doc = "Bit 4 - Jack output detect pending interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn jack_det_oirq(&mut self) -> JACK_DET_OIRQ_W<4> {
        JACK_DET_OIRQ_W::new(self)
    }
    #[doc = "Bits 13:14 - After MIC DATA data is received, the first N-data will be discarded."]
    #[inline(always)]
    #[must_use]
    pub fn mdata_discard(&mut self) -> MDATA_DISCARD_W<13> {
        MDATA_DISCARD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HMIC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hmic_sts](index.html) module"]
pub struct HMIC_STS_SPEC;
impl crate::RegisterSpec for HMIC_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hmic_sts::R](R) reader structure"]
impl crate::Readable for HMIC_STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hmic_sts::W](W) writer structure"]
impl crate::Writable for HMIC_STS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x19;
}
#[doc = "`reset()` method sets hmic_sts to value 0x6000"]
impl crate::Resettable for HMIC_STS_SPEC {
    const RESET_VALUE: Self::Ux = 0x6000;
}
