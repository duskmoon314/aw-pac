#[doc = "Register `tve_notch_filter_frequency` reader"]
pub type R = crate::R<TVE_NOTCH_FILTER_FREQUENCY_SPEC>;
#[doc = "Register `tve_notch_filter_frequency` writer"]
pub type W = crate::W<TVE_NOTCH_FILTER_FREQUENCY_SPEC>;
#[doc = "Field `notch_freq` reader - Luma notch filter center frequency selection\n\nThese bits select the luma notch filter (which is a band-reject filter) center frequency. In two of the selections, the filter width affects also the selection of the center frequency."]
pub type NOTCH_FREQ_R = crate::FieldReader<NOTCH_FREQ_A>;
#[doc = "Luma notch filter center frequency selection\n\nThese bits select the luma notch filter (which is a band-reject filter) center frequency. In two of the selections, the filter width affects also the selection of the center frequency.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NOTCH_FREQ_A {
    #[doc = "0: 1.1875"]
    _1_1875 = 0,
    #[doc = "1: 1.1406"]
    _1_1406 = 1,
    #[doc = "2: 1.0938. When notch_wide value is B'1' (this selection is proper for CCIR-NTSC), or 1.0000 when notch_wide value is B'0'."]
    _1_0938 = 2,
    #[doc = "3: 0.9922. This selection is proper for NTSC with square pixels."]
    _0_9922 = 3,
    #[doc = "4: 0.9531. This selection is proper for PAL with square pixel."]
    _0_9531 = 4,
    #[doc = "5: 0.8359 when notch_wide value is B'1' (this selection is proper for CCIR-PAL), or 0.7734 when notch_wide value is B'0'."]
    _0_8359 = 5,
    #[doc = "6: 0.7813"]
    _0_7813 = 6,
    #[doc = "7: 0.7188"]
    _0_7188 = 7,
}
impl From<NOTCH_FREQ_A> for u8 {
    #[inline(always)]
    fn from(variant: NOTCH_FREQ_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NOTCH_FREQ_A {
    type Ux = u8;
}
impl NOTCH_FREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NOTCH_FREQ_A {
        match self.bits {
            0 => NOTCH_FREQ_A::_1_1875,
            1 => NOTCH_FREQ_A::_1_1406,
            2 => NOTCH_FREQ_A::_1_0938,
            3 => NOTCH_FREQ_A::_0_9922,
            4 => NOTCH_FREQ_A::_0_9531,
            5 => NOTCH_FREQ_A::_0_8359,
            6 => NOTCH_FREQ_A::_0_7813,
            7 => NOTCH_FREQ_A::_0_7188,
            _ => unreachable!(),
        }
    }
    #[doc = "1.1875"]
    #[inline(always)]
    pub fn is_1_1875(&self) -> bool {
        *self == NOTCH_FREQ_A::_1_1875
    }
    #[doc = "1.1406"]
    #[inline(always)]
    pub fn is_1_1406(&self) -> bool {
        *self == NOTCH_FREQ_A::_1_1406
    }
    #[doc = "1.0938. When notch_wide value is B'1' (this selection is proper for CCIR-NTSC), or 1.0000 when notch_wide value is B'0'."]
    #[inline(always)]
    pub fn is_1_0938(&self) -> bool {
        *self == NOTCH_FREQ_A::_1_0938
    }
    #[doc = "0.9922. This selection is proper for NTSC with square pixels."]
    #[inline(always)]
    pub fn is_0_9922(&self) -> bool {
        *self == NOTCH_FREQ_A::_0_9922
    }
    #[doc = "0.9531. This selection is proper for PAL with square pixel."]
    #[inline(always)]
    pub fn is_0_9531(&self) -> bool {
        *self == NOTCH_FREQ_A::_0_9531
    }
    #[doc = "0.8359 when notch_wide value is B'1' (this selection is proper for CCIR-PAL), or 0.7734 when notch_wide value is B'0'."]
    #[inline(always)]
    pub fn is_0_8359(&self) -> bool {
        *self == NOTCH_FREQ_A::_0_8359
    }
    #[doc = "0.7813"]
    #[inline(always)]
    pub fn is_0_7813(&self) -> bool {
        *self == NOTCH_FREQ_A::_0_7813
    }
    #[doc = "0.7188"]
    #[inline(always)]
    pub fn is_0_7188(&self) -> bool {
        *self == NOTCH_FREQ_A::_0_7188
    }
}
#[doc = "Field `notch_freq` writer - Luma notch filter center frequency selection\n\nThese bits select the luma notch filter (which is a band-reject filter) center frequency. In two of the selections, the filter width affects also the selection of the center frequency."]
pub type NOTCH_FREQ_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, NOTCH_FREQ_A>;
impl<'a, REG> NOTCH_FREQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.1875"]
    #[inline(always)]
    pub fn _1_1875(self) -> &'a mut crate::W<REG> {
        self.variant(NOTCH_FREQ_A::_1_1875)
    }
    #[doc = "1.1406"]
    #[inline(always)]
    pub fn _1_1406(self) -> &'a mut crate::W<REG> {
        self.variant(NOTCH_FREQ_A::_1_1406)
    }
    #[doc = "1.0938. When notch_wide value is B'1' (this selection is proper for CCIR-NTSC), or 1.0000 when notch_wide value is B'0'."]
    #[inline(always)]
    pub fn _1_0938(self) -> &'a mut crate::W<REG> {
        self.variant(NOTCH_FREQ_A::_1_0938)
    }
    #[doc = "0.9922. This selection is proper for NTSC with square pixels."]
    #[inline(always)]
    pub fn _0_9922(self) -> &'a mut crate::W<REG> {
        self.variant(NOTCH_FREQ_A::_0_9922)
    }
    #[doc = "0.9531. This selection is proper for PAL with square pixel."]
    #[inline(always)]
    pub fn _0_9531(self) -> &'a mut crate::W<REG> {
        self.variant(NOTCH_FREQ_A::_0_9531)
    }
    #[doc = "0.8359 when notch_wide value is B'1' (this selection is proper for CCIR-PAL), or 0.7734 when notch_wide value is B'0'."]
    #[inline(always)]
    pub fn _0_8359(self) -> &'a mut crate::W<REG> {
        self.variant(NOTCH_FREQ_A::_0_8359)
    }
    #[doc = "0.7813"]
    #[inline(always)]
    pub fn _0_7813(self) -> &'a mut crate::W<REG> {
        self.variant(NOTCH_FREQ_A::_0_7813)
    }
    #[doc = "0.7188"]
    #[inline(always)]
    pub fn _0_7188(self) -> &'a mut crate::W<REG> {
        self.variant(NOTCH_FREQ_A::_0_7188)
    }
}
impl R {
    #[doc = "Bits 0:2 - Luma notch filter center frequency selection\n\nThese bits select the luma notch filter (which is a band-reject filter) center frequency. In two of the selections, the filter width affects also the selection of the center frequency."]
    #[inline(always)]
    pub fn notch_freq(&self) -> NOTCH_FREQ_R {
        NOTCH_FREQ_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Luma notch filter center frequency selection\n\nThese bits select the luma notch filter (which is a band-reject filter) center frequency. In two of the selections, the filter width affects also the selection of the center frequency."]
    #[inline(always)]
    #[must_use]
    pub fn notch_freq(&mut self) -> NOTCH_FREQ_W<TVE_NOTCH_FILTER_FREQUENCY_SPEC> {
        NOTCH_FREQ_W::new(self, 0)
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
#[doc = "TV Encoder Notch Filter Frequency Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_notch_filter_frequency::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_notch_filter_frequency::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TVE_NOTCH_FILTER_FREQUENCY_SPEC;
impl crate::RegisterSpec for TVE_NOTCH_FILTER_FREQUENCY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tve_notch_filter_frequency::R`](R) reader structure"]
impl crate::Readable for TVE_NOTCH_FILTER_FREQUENCY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tve_notch_filter_frequency::W`](W) writer structure"]
impl crate::Writable for TVE_NOTCH_FILTER_FREQUENCY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_notch_filter_frequency to value 0x02"]
impl crate::Resettable for TVE_NOTCH_FILTER_FREQUENCY_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
