#[doc = "Register `tve_video_chroma_bw_comp_gain` reader"]
pub type R = crate::R<TVE_VIDEO_CHROMA_BW_COMP_GAIN_SPEC>;
#[doc = "Register `tve_video_chroma_bw_comp_gain` writer"]
pub type W = crate::W<TVE_VIDEO_CHROMA_BW_COMP_GAIN_SPEC>;
#[doc = "Field `comp_ch_gain` reader - Chroma gain selection for the composite video signal.\n\nThese bits specify the gain of the chroma signal for composing with the luma signal to generate the composite video signal:"]
pub type COMP_CH_GAIN_R = crate::FieldReader<COMP_CH_GAIN_A>;
#[doc = "Chroma gain selection for the composite video signal.\n\nThese bits specify the gain of the chroma signal for composing with the luma signal to generate the composite video signal:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP_CH_GAIN_A {
    #[doc = "0: 100%"]
    _100 = 0,
    #[doc = "1: 25%"]
    _25 = 1,
    #[doc = "2: 50%"]
    _50 = 2,
    #[doc = "3: 75%"]
    _75 = 3,
}
impl From<COMP_CH_GAIN_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP_CH_GAIN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COMP_CH_GAIN_A {
    type Ux = u8;
}
impl COMP_CH_GAIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMP_CH_GAIN_A {
        match self.bits {
            0 => COMP_CH_GAIN_A::_100,
            1 => COMP_CH_GAIN_A::_25,
            2 => COMP_CH_GAIN_A::_50,
            3 => COMP_CH_GAIN_A::_75,
            _ => unreachable!(),
        }
    }
    #[doc = "100%"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == COMP_CH_GAIN_A::_100
    }
    #[doc = "25%"]
    #[inline(always)]
    pub fn is_25(&self) -> bool {
        *self == COMP_CH_GAIN_A::_25
    }
    #[doc = "50%"]
    #[inline(always)]
    pub fn is_50(&self) -> bool {
        *self == COMP_CH_GAIN_A::_50
    }
    #[doc = "75%"]
    #[inline(always)]
    pub fn is_75(&self) -> bool {
        *self == COMP_CH_GAIN_A::_75
    }
}
#[doc = "Field `comp_ch_gain` writer - Chroma gain selection for the composite video signal.\n\nThese bits specify the gain of the chroma signal for composing with the luma signal to generate the composite video signal:"]
pub type COMP_CH_GAIN_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, COMP_CH_GAIN_A>;
impl<'a, REG> COMP_CH_GAIN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "100%"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(COMP_CH_GAIN_A::_100)
    }
    #[doc = "25%"]
    #[inline(always)]
    pub fn _25(self) -> &'a mut crate::W<REG> {
        self.variant(COMP_CH_GAIN_A::_25)
    }
    #[doc = "50%"]
    #[inline(always)]
    pub fn _50(self) -> &'a mut crate::W<REG> {
        self.variant(COMP_CH_GAIN_A::_50)
    }
    #[doc = "75%"]
    #[inline(always)]
    pub fn _75(self) -> &'a mut crate::W<REG> {
        self.variant(COMP_CH_GAIN_A::_75)
    }
}
#[doc = "Field `chroma_bw` reader - Chroma filter bandwidth selection\n\nThis bit specifies whether the bandwidth of the chroma filter is:"]
pub type CHROMA_BW_R = crate::FieldReader<CHROMA_BW_A>;
#[doc = "Chroma filter bandwidth selection\n\nThis bit specifies whether the bandwidth of the chroma filter is:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHROMA_BW_A {
    #[doc = "0: Narrow width 0.6 MHz"]
    N_ARROW = 0,
    #[doc = "1: Wide width 1.2 MHz"]
    W_IDE = 1,
    #[doc = "2: Extra width 1.8 MHz"]
    E_XTRA = 2,
    #[doc = "3: Ultra width 2.5 MHz"]
    U_LTRA = 3,
}
impl From<CHROMA_BW_A> for u8 {
    #[inline(always)]
    fn from(variant: CHROMA_BW_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CHROMA_BW_A {
    type Ux = u8;
}
impl CHROMA_BW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHROMA_BW_A {
        match self.bits {
            0 => CHROMA_BW_A::N_ARROW,
            1 => CHROMA_BW_A::W_IDE,
            2 => CHROMA_BW_A::E_XTRA,
            3 => CHROMA_BW_A::U_LTRA,
            _ => unreachable!(),
        }
    }
    #[doc = "Narrow width 0.6 MHz"]
    #[inline(always)]
    pub fn is_n_arrow(&self) -> bool {
        *self == CHROMA_BW_A::N_ARROW
    }
    #[doc = "Wide width 1.2 MHz"]
    #[inline(always)]
    pub fn is_w_ide(&self) -> bool {
        *self == CHROMA_BW_A::W_IDE
    }
    #[doc = "Extra width 1.8 MHz"]
    #[inline(always)]
    pub fn is_e_xtra(&self) -> bool {
        *self == CHROMA_BW_A::E_XTRA
    }
    #[doc = "Ultra width 2.5 MHz"]
    #[inline(always)]
    pub fn is_u_ltra(&self) -> bool {
        *self == CHROMA_BW_A::U_LTRA
    }
}
#[doc = "Field `chroma_bw` writer - Chroma filter bandwidth selection\n\nThis bit specifies whether the bandwidth of the chroma filter is:"]
pub type CHROMA_BW_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CHROMA_BW_A>;
impl<'a, REG> CHROMA_BW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Narrow width 0.6 MHz"]
    #[inline(always)]
    pub fn n_arrow(self) -> &'a mut crate::W<REG> {
        self.variant(CHROMA_BW_A::N_ARROW)
    }
    #[doc = "Wide width 1.2 MHz"]
    #[inline(always)]
    pub fn w_ide(self) -> &'a mut crate::W<REG> {
        self.variant(CHROMA_BW_A::W_IDE)
    }
    #[doc = "Extra width 1.8 MHz"]
    #[inline(always)]
    pub fn e_xtra(self) -> &'a mut crate::W<REG> {
        self.variant(CHROMA_BW_A::E_XTRA)
    }
    #[doc = "Ultra width 2.5 MHz"]
    #[inline(always)]
    pub fn u_ltra(self) -> &'a mut crate::W<REG> {
        self.variant(CHROMA_BW_A::U_LTRA)
    }
}
impl R {
    #[doc = "Bits 0:1 - Chroma gain selection for the composite video signal.\n\nThese bits specify the gain of the chroma signal for composing with the luma signal to generate the composite video signal:"]
    #[inline(always)]
    pub fn comp_ch_gain(&self) -> COMP_CH_GAIN_R {
        COMP_CH_GAIN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:17 - Chroma filter bandwidth selection\n\nThis bit specifies whether the bandwidth of the chroma filter is:"]
    #[inline(always)]
    pub fn chroma_bw(&self) -> CHROMA_BW_R {
        CHROMA_BW_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Chroma gain selection for the composite video signal.\n\nThese bits specify the gain of the chroma signal for composing with the luma signal to generate the composite video signal:"]
    #[inline(always)]
    #[must_use]
    pub fn comp_ch_gain(&mut self) -> COMP_CH_GAIN_W<TVE_VIDEO_CHROMA_BW_COMP_GAIN_SPEC> {
        COMP_CH_GAIN_W::new(self, 0)
    }
    #[doc = "Bits 16:17 - Chroma filter bandwidth selection\n\nThis bit specifies whether the bandwidth of the chroma filter is:"]
    #[inline(always)]
    #[must_use]
    pub fn chroma_bw(&mut self) -> CHROMA_BW_W<TVE_VIDEO_CHROMA_BW_COMP_GAIN_SPEC> {
        CHROMA_BW_W::new(self, 16)
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
#[doc = "TV Encoder Video Chroma BW and CompGain Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_video_chroma_bw_comp_gain::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_video_chroma_bw_comp_gain::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TVE_VIDEO_CHROMA_BW_COMP_GAIN_SPEC;
impl crate::RegisterSpec for TVE_VIDEO_CHROMA_BW_COMP_GAIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tve_video_chroma_bw_comp_gain::R`](R) reader structure"]
impl crate::Readable for TVE_VIDEO_CHROMA_BW_COMP_GAIN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tve_video_chroma_bw_comp_gain::W`](W) writer structure"]
impl crate::Writable for TVE_VIDEO_CHROMA_BW_COMP_GAIN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_video_chroma_bw_comp_gain to value 0"]
impl crate::Resettable for TVE_VIDEO_CHROMA_BW_COMP_GAIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
