#[doc = "Register `tve_configuration0` reader"]
pub type R = crate::R<TVE_CONFIGURATION0_SPEC>;
#[doc = "Register `tve_configuration0` writer"]
pub type W = crate::W<TVE_CONFIGURATION0_SPEC>;
#[doc = "Field `uv_order` reader - This bit selects if the sample order at the chroma input to the Video Encoder is Cb first (i.e. Cb 0 Cr 0 Cb 1 Cr 1) or Cr first (i.e. Cr 0 Cb 0 Cr 1 Cb 1)."]
pub type UV_ORDER_R = crate::BitReader<UV_ORDER_A>;
#[doc = "This bit selects if the sample order at the chroma input to the Video Encoder is Cb first (i.e. Cb 0 Cr 0 Cb 1 Cr 1) or Cr first (i.e. Cr 0 Cb 0 Cr 1 Cb 1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UV_ORDER_A {
    #[doc = "0: The chroma sample input order is Cb first"]
    C_B = 0,
    #[doc = "1: The chroma sample input order is Cr first"]
    C_R = 1,
}
impl From<UV_ORDER_A> for bool {
    #[inline(always)]
    fn from(variant: UV_ORDER_A) -> Self {
        variant as u8 != 0
    }
}
impl UV_ORDER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UV_ORDER_A {
        match self.bits {
            false => UV_ORDER_A::C_B,
            true => UV_ORDER_A::C_R,
        }
    }
    #[doc = "The chroma sample input order is Cb first"]
    #[inline(always)]
    pub fn is_c_b(&self) -> bool {
        *self == UV_ORDER_A::C_B
    }
    #[doc = "The chroma sample input order is Cr first"]
    #[inline(always)]
    pub fn is_c_r(&self) -> bool {
        *self == UV_ORDER_A::C_R
    }
}
#[doc = "Field `uv_order` writer - This bit selects if the sample order at the chroma input to the Video Encoder is Cb first (i.e. Cb 0 Cr 0 Cb 1 Cr 1) or Cr first (i.e. Cr 0 Cb 0 Cr 1 Cb 1)."]
pub type UV_ORDER_W<'a, REG> = crate::BitWriter<'a, REG, UV_ORDER_A>;
impl<'a, REG> UV_ORDER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The chroma sample input order is Cb first"]
    #[inline(always)]
    pub fn c_b(self) -> &'a mut crate::W<REG> {
        self.variant(UV_ORDER_A::C_B)
    }
    #[doc = "The chroma sample input order is Cr first"]
    #[inline(always)]
    pub fn c_r(self) -> &'a mut crate::W<REG> {
        self.variant(UV_ORDER_A::C_R)
    }
}
#[doc = "Field `invert_top` reader - Field parity input signal (top_field) polarity selection.\n\nThis bit selects whether the top field is indicated by a high level of the field parity signal or by the low level. The bit is applicable both when the Video Encoder is the sync master and when the Video Encoder is the sync slave."]
pub type INVERT_TOP_R = crate::BitReader<INVERT_TOP_A>;
#[doc = "Field parity input signal (top_field) polarity selection.\n\nThis bit selects whether the top field is indicated by a high level of the field parity signal or by the low level. The bit is applicable both when the Video Encoder is the sync master and when the Video Encoder is the sync slave.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INVERT_TOP_A {
    #[doc = "0: Top field is indicated by low level"]
    LOW = 0,
    #[doc = "1: Top field is indicated by high level"]
    HIGH = 1,
}
impl From<INVERT_TOP_A> for bool {
    #[inline(always)]
    fn from(variant: INVERT_TOP_A) -> Self {
        variant as u8 != 0
    }
}
impl INVERT_TOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INVERT_TOP_A {
        match self.bits {
            false => INVERT_TOP_A::LOW,
            true => INVERT_TOP_A::HIGH,
        }
    }
    #[doc = "Top field is indicated by low level"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == INVERT_TOP_A::LOW
    }
    #[doc = "Top field is indicated by high level"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == INVERT_TOP_A::HIGH
    }
}
#[doc = "Field `invert_top` writer - Field parity input signal (top_field) polarity selection.\n\nThis bit selects whether the top field is indicated by a high level of the field parity signal or by the low level. The bit is applicable both when the Video Encoder is the sync master and when the Video Encoder is the sync slave."]
pub type INVERT_TOP_W<'a, REG> = crate::BitWriter<'a, REG, INVERT_TOP_A>;
impl<'a, REG> INVERT_TOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Top field is indicated by low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(INVERT_TOP_A::LOW)
    }
    #[doc = "Top field is indicated by high level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(INVERT_TOP_A::HIGH)
    }
}
impl R {
    #[doc = "Bit 0 - This bit selects if the sample order at the chroma input to the Video Encoder is Cb first (i.e. Cb 0 Cr 0 Cb 1 Cr 1) or Cr first (i.e. Cr 0 Cb 0 Cr 1 Cb 1)."]
    #[inline(always)]
    pub fn uv_order(&self) -> UV_ORDER_R {
        UV_ORDER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Field parity input signal (top_field) polarity selection.\n\nThis bit selects whether the top field is indicated by a high level of the field parity signal or by the low level. The bit is applicable both when the Video Encoder is the sync master and when the Video Encoder is the sync slave."]
    #[inline(always)]
    pub fn invert_top(&self) -> INVERT_TOP_R {
        INVERT_TOP_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit selects if the sample order at the chroma input to the Video Encoder is Cb first (i.e. Cb 0 Cr 0 Cb 1 Cr 1) or Cr first (i.e. Cr 0 Cb 0 Cr 1 Cb 1)."]
    #[inline(always)]
    #[must_use]
    pub fn uv_order(&mut self) -> UV_ORDER_W<TVE_CONFIGURATION0_SPEC> {
        UV_ORDER_W::new(self, 0)
    }
    #[doc = "Bit 8 - Field parity input signal (top_field) polarity selection.\n\nThis bit selects whether the top field is indicated by a high level of the field parity signal or by the low level. The bit is applicable both when the Video Encoder is the sync master and when the Video Encoder is the sync slave."]
    #[inline(always)]
    #[must_use]
    pub fn invert_top(&mut self) -> INVERT_TOP_W<TVE_CONFIGURATION0_SPEC> {
        INVERT_TOP_W::new(self, 8)
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
#[doc = "TV Encoder Configuration Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_configuration0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_configuration0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TVE_CONFIGURATION0_SPEC;
impl crate::RegisterSpec for TVE_CONFIGURATION0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tve_configuration0::R`](R) reader structure"]
impl crate::Readable for TVE_CONFIGURATION0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tve_configuration0::W`](W) writer structure"]
impl crate::Writable for TVE_CONFIGURATION0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_configuration0 to value 0"]
impl crate::Resettable for TVE_CONFIGURATION0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
