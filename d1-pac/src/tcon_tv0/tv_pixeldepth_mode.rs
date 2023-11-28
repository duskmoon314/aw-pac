#[doc = "Register `tv_pixeldepth_mode` reader"]
pub type R = crate::R<TV_PIXELDEPTH_MODE_SPEC>;
#[doc = "Register `tv_pixeldepth_mode` writer"]
pub type W = crate::W<TV_PIXELDEPTH_MODE_SPEC>;
#[doc = "Field `colorbar_pd_mode` reader - Colorbar Pixeldepth mode"]
pub type COLORBAR_PD_MODE_R = crate::BitReader<COLORBAR_PD_MODE_A>;
#[doc = "Colorbar Pixeldepth mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COLORBAR_PD_MODE_A {
    #[doc = "0: 8 bit mode\n\nWhen data source is the embedded colorbar, the 8-bit colorbar pattern is transmitted."]
    B_IT8 = 0,
    #[doc = "1: 10 bit mode\n\nWhen data source is the embedded colorbar, the 10-bit colorbar pattern is transmitted."]
    B_IT10 = 1,
}
impl From<COLORBAR_PD_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: COLORBAR_PD_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl COLORBAR_PD_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COLORBAR_PD_MODE_A {
        match self.bits {
            false => COLORBAR_PD_MODE_A::B_IT8,
            true => COLORBAR_PD_MODE_A::B_IT10,
        }
    }
    #[doc = "8 bit mode\n\nWhen data source is the embedded colorbar, the 8-bit colorbar pattern is transmitted."]
    #[inline(always)]
    pub fn is_b_it8(&self) -> bool {
        *self == COLORBAR_PD_MODE_A::B_IT8
    }
    #[doc = "10 bit mode\n\nWhen data source is the embedded colorbar, the 10-bit colorbar pattern is transmitted."]
    #[inline(always)]
    pub fn is_b_it10(&self) -> bool {
        *self == COLORBAR_PD_MODE_A::B_IT10
    }
}
#[doc = "Field `colorbar_pd_mode` writer - Colorbar Pixeldepth mode"]
pub type COLORBAR_PD_MODE_W<'a, REG> = crate::BitWriter<'a, REG, COLORBAR_PD_MODE_A>;
impl<'a, REG> COLORBAR_PD_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "8 bit mode\n\nWhen data source is the embedded colorbar, the 8-bit colorbar pattern is transmitted."]
    #[inline(always)]
    pub fn b_it8(self) -> &'a mut crate::W<REG> {
        self.variant(COLORBAR_PD_MODE_A::B_IT8)
    }
    #[doc = "10 bit mode\n\nWhen data source is the embedded colorbar, the 10-bit colorbar pattern is transmitted."]
    #[inline(always)]
    pub fn b_it10(self) -> &'a mut crate::W<REG> {
        self.variant(COLORBAR_PD_MODE_A::B_IT10)
    }
}
impl R {
    #[doc = "Bit 0 - Colorbar Pixeldepth mode"]
    #[inline(always)]
    pub fn colorbar_pd_mode(&self) -> COLORBAR_PD_MODE_R {
        COLORBAR_PD_MODE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Colorbar Pixeldepth mode"]
    #[inline(always)]
    #[must_use]
    pub fn colorbar_pd_mode(&mut self) -> COLORBAR_PD_MODE_W<TV_PIXELDEPTH_MODE_SPEC> {
        COLORBAR_PD_MODE_W::new(self, 0)
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
#[doc = "TV Pixeldepth Mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_pixeldepth_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_pixeldepth_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TV_PIXELDEPTH_MODE_SPEC;
impl crate::RegisterSpec for TV_PIXELDEPTH_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tv_pixeldepth_mode::R`](R) reader structure"]
impl crate::Readable for TV_PIXELDEPTH_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tv_pixeldepth_mode::W`](W) writer structure"]
impl crate::Writable for TV_PIXELDEPTH_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tv_pixeldepth_mode to value 0"]
impl crate::Resettable for TV_PIXELDEPTH_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
