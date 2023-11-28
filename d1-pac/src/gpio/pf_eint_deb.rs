#[doc = "Register `pf_eint_deb` reader"]
pub type R = crate::R<PF_EINT_DEB_SPEC>;
#[doc = "Register `pf_eint_deb` writer"]
pub type W = crate::W<PF_EINT_DEB_SPEC>;
#[doc = "Field `pio_int_clk_select` reader - PIO Interrupt Clock Select"]
pub type PIO_INT_CLK_SELECT_R = crate::BitReader<PIO_INT_CLK_SELECT_A>;
#[doc = "PIO Interrupt Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIO_INT_CLK_SELECT_A {
    #[doc = "0: `0`"]
    LOSC_32K_HZ = 0,
    #[doc = "1: `1`"]
    HOSC_24M_HZ = 1,
}
impl From<PIO_INT_CLK_SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: PIO_INT_CLK_SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO_INT_CLK_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIO_INT_CLK_SELECT_A {
        match self.bits {
            false => PIO_INT_CLK_SELECT_A::LOSC_32K_HZ,
            true => PIO_INT_CLK_SELECT_A::HOSC_24M_HZ,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_losc_32k_hz(&self) -> bool {
        *self == PIO_INT_CLK_SELECT_A::LOSC_32K_HZ
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_hosc_24m_hz(&self) -> bool {
        *self == PIO_INT_CLK_SELECT_A::HOSC_24M_HZ
    }
}
#[doc = "Field `pio_int_clk_select` writer - PIO Interrupt Clock Select"]
pub type PIO_INT_CLK_SELECT_W<'a, REG> = crate::BitWriter<'a, REG, PIO_INT_CLK_SELECT_A>;
impl<'a, REG> PIO_INT_CLK_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn losc_32k_hz(self) -> &'a mut crate::W<REG> {
        self.variant(PIO_INT_CLK_SELECT_A::LOSC_32K_HZ)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn hosc_24m_hz(self) -> &'a mut crate::W<REG> {
        self.variant(PIO_INT_CLK_SELECT_A::HOSC_24M_HZ)
    }
}
#[doc = "Field `deb_clk_pre_scale` reader - Debounce Clock Pre_scale n"]
pub type DEB_CLK_PRE_SCALE_R = crate::FieldReader;
#[doc = "Field `deb_clk_pre_scale` writer - Debounce Clock Pre_scale n"]
pub type DEB_CLK_PRE_SCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - PIO Interrupt Clock Select"]
    #[inline(always)]
    pub fn pio_int_clk_select(&self) -> PIO_INT_CLK_SELECT_R {
        PIO_INT_CLK_SELECT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - Debounce Clock Pre_scale n"]
    #[inline(always)]
    pub fn deb_clk_pre_scale(&self) -> DEB_CLK_PRE_SCALE_R {
        DEB_CLK_PRE_SCALE_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PIO Interrupt Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn pio_int_clk_select(&mut self) -> PIO_INT_CLK_SELECT_W<PF_EINT_DEB_SPEC> {
        PIO_INT_CLK_SELECT_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Debounce Clock Pre_scale n"]
    #[inline(always)]
    #[must_use]
    pub fn deb_clk_pre_scale(&mut self) -> DEB_CLK_PRE_SCALE_W<PF_EINT_DEB_SPEC> {
        DEB_CLK_PRE_SCALE_W::new(self, 4)
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
#[doc = "PF External Interrupt Debounce Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pf_eint_deb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pf_eint_deb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PF_EINT_DEB_SPEC;
impl crate::RegisterSpec for PF_EINT_DEB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pf_eint_deb::R`](R) reader structure"]
impl crate::Readable for PF_EINT_DEB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pf_eint_deb::W`](W) writer structure"]
impl crate::Writable for PF_EINT_DEB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pf_eint_deb to value 0"]
impl crate::Resettable for PF_EINT_DEB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
