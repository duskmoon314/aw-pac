#[doc = "Register `tv_data_io_tri0` reader"]
pub type R = crate::R<TV_DATA_IO_TRI0_SPEC>;
#[doc = "Register `tv_data_io_tri0` writer"]
pub type W = crate::W<TV_DATA_IO_TRI0_SPEC>;
#[doc = "Field `g_y_ch_data_out_tri_en` reader - G Y Channel Data Output Trigger Enable"]
pub type G_Y_CH_DATA_OUT_TRI_EN_R = crate::FieldReader<G_Y_CH_DATA_OUT_TRI_EN_A>;
#[doc = "G Y Channel Data Output Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum G_Y_CH_DATA_OUT_TRI_EN_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<G_Y_CH_DATA_OUT_TRI_EN_A> for u16 {
    #[inline(always)]
    fn from(variant: G_Y_CH_DATA_OUT_TRI_EN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for G_Y_CH_DATA_OUT_TRI_EN_A {
    type Ux = u16;
}
impl G_Y_CH_DATA_OUT_TRI_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<G_Y_CH_DATA_OUT_TRI_EN_A> {
        match self.bits {
            0 => Some(G_Y_CH_DATA_OUT_TRI_EN_A::DISABLE),
            1 => Some(G_Y_CH_DATA_OUT_TRI_EN_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == G_Y_CH_DATA_OUT_TRI_EN_A::DISABLE
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == G_Y_CH_DATA_OUT_TRI_EN_A::ENABLE
    }
}
#[doc = "Field `g_y_ch_data_out_tri_en` writer - G Y Channel Data Output Trigger Enable"]
pub type G_Y_CH_DATA_OUT_TRI_EN_W<'a, REG> =
    crate::FieldWriter<'a, REG, 10, G_Y_CH_DATA_OUT_TRI_EN_A>;
impl<'a, REG> G_Y_CH_DATA_OUT_TRI_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(G_Y_CH_DATA_OUT_TRI_EN_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(G_Y_CH_DATA_OUT_TRI_EN_A::ENABLE)
    }
}
#[doc = "Field `r_cb_ch_data_out_tri_en` reader - R CB Channel Data Output Trigger Enable"]
pub type R_CB_CH_DATA_OUT_TRI_EN_R = crate::FieldReader<R_CB_CH_DATA_OUT_TRI_EN_A>;
#[doc = "R CB Channel Data Output Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum R_CB_CH_DATA_OUT_TRI_EN_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<R_CB_CH_DATA_OUT_TRI_EN_A> for u16 {
    #[inline(always)]
    fn from(variant: R_CB_CH_DATA_OUT_TRI_EN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for R_CB_CH_DATA_OUT_TRI_EN_A {
    type Ux = u16;
}
impl R_CB_CH_DATA_OUT_TRI_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<R_CB_CH_DATA_OUT_TRI_EN_A> {
        match self.bits {
            0 => Some(R_CB_CH_DATA_OUT_TRI_EN_A::DISABLE),
            1 => Some(R_CB_CH_DATA_OUT_TRI_EN_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == R_CB_CH_DATA_OUT_TRI_EN_A::DISABLE
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == R_CB_CH_DATA_OUT_TRI_EN_A::ENABLE
    }
}
#[doc = "Field `r_cb_ch_data_out_tri_en` writer - R CB Channel Data Output Trigger Enable"]
pub type R_CB_CH_DATA_OUT_TRI_EN_W<'a, REG> =
    crate::FieldWriter<'a, REG, 10, R_CB_CH_DATA_OUT_TRI_EN_A>;
impl<'a, REG> R_CB_CH_DATA_OUT_TRI_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(R_CB_CH_DATA_OUT_TRI_EN_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(R_CB_CH_DATA_OUT_TRI_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:9 - G Y Channel Data Output Trigger Enable"]
    #[inline(always)]
    pub fn g_y_ch_data_out_tri_en(&self) -> G_Y_CH_DATA_OUT_TRI_EN_R {
        G_Y_CH_DATA_OUT_TRI_EN_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - R CB Channel Data Output Trigger Enable"]
    #[inline(always)]
    pub fn r_cb_ch_data_out_tri_en(&self) -> R_CB_CH_DATA_OUT_TRI_EN_R {
        R_CB_CH_DATA_OUT_TRI_EN_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - G Y Channel Data Output Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn g_y_ch_data_out_tri_en(&mut self) -> G_Y_CH_DATA_OUT_TRI_EN_W<TV_DATA_IO_TRI0_SPEC> {
        G_Y_CH_DATA_OUT_TRI_EN_W::new(self, 0)
    }
    #[doc = "Bits 16:25 - R CB Channel Data Output Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn r_cb_ch_data_out_tri_en(&mut self) -> R_CB_CH_DATA_OUT_TRI_EN_W<TV_DATA_IO_TRI0_SPEC> {
        R_CB_CH_DATA_OUT_TRI_EN_W::new(self, 16)
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
#[doc = "TCON Data IO Enable Control0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_data_io_tri0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_data_io_tri0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TV_DATA_IO_TRI0_SPEC;
impl crate::RegisterSpec for TV_DATA_IO_TRI0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tv_data_io_tri0::R`](R) reader structure"]
impl crate::Readable for TV_DATA_IO_TRI0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tv_data_io_tri0::W`](W) writer structure"]
impl crate::Writable for TV_DATA_IO_TRI0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tv_data_io_tri0 to value 0"]
impl crate::Resettable for TV_DATA_IO_TRI0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
