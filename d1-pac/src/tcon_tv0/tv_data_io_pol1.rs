#[doc = "Register `tv_data_io_pol1` reader"]
pub type R = crate::R<TV_DATA_IO_POL1_SPEC>;
#[doc = "Register `tv_data_io_pol1` writer"]
pub type W = crate::W<TV_DATA_IO_POL1_SPEC>;
#[doc = "Field `b_cr_ch_data_inv` reader - B CR CHANNE DATA INV"]
pub type B_CR_CH_DATA_INV_R = crate::FieldReader<B_CR_CH_DATA_INV_A>;
#[doc = "B CR CHANNE DATA INV\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum B_CR_CH_DATA_INV_A {
    #[doc = "0: normal polarity"]
    NORMAL = 0,
    #[doc = "1: invert the specify output"]
    INVERT = 1,
}
impl From<B_CR_CH_DATA_INV_A> for u16 {
    #[inline(always)]
    fn from(variant: B_CR_CH_DATA_INV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for B_CR_CH_DATA_INV_A {
    type Ux = u16;
}
impl B_CR_CH_DATA_INV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<B_CR_CH_DATA_INV_A> {
        match self.bits {
            0 => Some(B_CR_CH_DATA_INV_A::NORMAL),
            1 => Some(B_CR_CH_DATA_INV_A::INVERT),
            _ => None,
        }
    }
    #[doc = "normal polarity"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == B_CR_CH_DATA_INV_A::NORMAL
    }
    #[doc = "invert the specify output"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == B_CR_CH_DATA_INV_A::INVERT
    }
}
#[doc = "Field `b_cr_ch_data_inv` writer - B CR CHANNE DATA INV"]
pub type B_CR_CH_DATA_INV_W<'a, REG> = crate::FieldWriter<'a, REG, 10, B_CR_CH_DATA_INV_A>;
impl<'a, REG> B_CR_CH_DATA_INV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "normal polarity"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(B_CR_CH_DATA_INV_A::NORMAL)
    }
    #[doc = "invert the specify output"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut crate::W<REG> {
        self.variant(B_CR_CH_DATA_INV_A::INVERT)
    }
}
impl R {
    #[doc = "Bits 16:25 - B CR CHANNE DATA INV"]
    #[inline(always)]
    pub fn b_cr_ch_data_inv(&self) -> B_CR_CH_DATA_INV_R {
        B_CR_CH_DATA_INV_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:25 - B CR CHANNE DATA INV"]
    #[inline(always)]
    #[must_use]
    pub fn b_cr_ch_data_inv(&mut self) -> B_CR_CH_DATA_INV_W<TV_DATA_IO_POL1_SPEC> {
        B_CR_CH_DATA_INV_W::new(self, 16)
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
#[doc = "TCON Data IO Polarity Control1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_data_io_pol1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_data_io_pol1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TV_DATA_IO_POL1_SPEC;
impl crate::RegisterSpec for TV_DATA_IO_POL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tv_data_io_pol1::R`](R) reader structure"]
impl crate::Readable for TV_DATA_IO_POL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tv_data_io_pol1::W`](W) writer structure"]
impl crate::Writable for TV_DATA_IO_POL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tv_data_io_pol1 to value 0"]
impl crate::Resettable for TV_DATA_IO_POL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
