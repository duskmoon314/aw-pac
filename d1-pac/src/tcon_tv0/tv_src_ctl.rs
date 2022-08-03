#[doc = "Register `tv_src_ctl` reader"]
pub struct R(crate::R<TV_SRC_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TV_SRC_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TV_SRC_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TV_SRC_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tv_src_ctl` writer"]
pub struct W(crate::W<TV_SRC_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TV_SRC_CTL_SPEC>;
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
impl From<crate::W<TV_SRC_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TV_SRC_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tv_src_sel` reader - TV Source Select"]
pub type TV_SRC_SEL_R = crate::FieldReader<u8, TV_SRC_SEL_A>;
#[doc = "TV Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TV_SRC_SEL_A {
    #[doc = "0: DE"]
    DE = 0,
    #[doc = "1: Color Check"]
    C_OLOR = 1,
    #[doc = "2: Grayscale Check"]
    G_RAYSCALE = 2,
    #[doc = "3: Black by White Check"]
    B_LACK_B_Y_W_HITE = 3,
    #[doc = "7: Gridding Check"]
    G_RIDDING = 7,
}
impl From<TV_SRC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TV_SRC_SEL_A) -> Self {
        variant as _
    }
}
impl TV_SRC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TV_SRC_SEL_A> {
        match self.bits {
            0 => Some(TV_SRC_SEL_A::DE),
            1 => Some(TV_SRC_SEL_A::C_OLOR),
            2 => Some(TV_SRC_SEL_A::G_RAYSCALE),
            3 => Some(TV_SRC_SEL_A::B_LACK_B_Y_W_HITE),
            7 => Some(TV_SRC_SEL_A::G_RIDDING),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DE`"]
    #[inline(always)]
    pub fn is_de(&self) -> bool {
        *self == TV_SRC_SEL_A::DE
    }
    #[doc = "Checks if the value of the field is `C_OLOR`"]
    #[inline(always)]
    pub fn is_c_olor(&self) -> bool {
        *self == TV_SRC_SEL_A::C_OLOR
    }
    #[doc = "Checks if the value of the field is `G_RAYSCALE`"]
    #[inline(always)]
    pub fn is_g_rayscale(&self) -> bool {
        *self == TV_SRC_SEL_A::G_RAYSCALE
    }
    #[doc = "Checks if the value of the field is `B_LACK_B_Y_W_HITE`"]
    #[inline(always)]
    pub fn is_b_lack_b_y_w_hite(&self) -> bool {
        *self == TV_SRC_SEL_A::B_LACK_B_Y_W_HITE
    }
    #[doc = "Checks if the value of the field is `G_RIDDING`"]
    #[inline(always)]
    pub fn is_g_ridding(&self) -> bool {
        *self == TV_SRC_SEL_A::G_RIDDING
    }
}
#[doc = "Field `tv_src_sel` writer - TV Source Select"]
pub type TV_SRC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TV_SRC_CTL_SPEC, u8, TV_SRC_SEL_A, 3, O>;
impl<'a, const O: u8> TV_SRC_SEL_W<'a, O> {
    #[doc = "DE"]
    #[inline(always)]
    pub fn de(self) -> &'a mut W {
        self.variant(TV_SRC_SEL_A::DE)
    }
    #[doc = "Color Check"]
    #[inline(always)]
    pub fn c_olor(self) -> &'a mut W {
        self.variant(TV_SRC_SEL_A::C_OLOR)
    }
    #[doc = "Grayscale Check"]
    #[inline(always)]
    pub fn g_rayscale(self) -> &'a mut W {
        self.variant(TV_SRC_SEL_A::G_RAYSCALE)
    }
    #[doc = "Black by White Check"]
    #[inline(always)]
    pub fn b_lack_b_y_w_hite(self) -> &'a mut W {
        self.variant(TV_SRC_SEL_A::B_LACK_B_Y_W_HITE)
    }
    #[doc = "Gridding Check"]
    #[inline(always)]
    pub fn g_ridding(self) -> &'a mut W {
        self.variant(TV_SRC_SEL_A::G_RIDDING)
    }
}
impl R {
    #[doc = "Bits 0:2 - TV Source Select"]
    #[inline(always)]
    pub fn tv_src_sel(&self) -> TV_SRC_SEL_R {
        TV_SRC_SEL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - TV Source Select"]
    #[inline(always)]
    pub fn tv_src_sel(&mut self) -> TV_SRC_SEL_W<0> {
        TV_SRC_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Source Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tv_src_ctl](index.html) module"]
pub struct TV_SRC_CTL_SPEC;
impl crate::RegisterSpec for TV_SRC_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tv_src_ctl::R](R) reader structure"]
impl crate::Readable for TV_SRC_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tv_src_ctl::W](W) writer structure"]
impl crate::Writable for TV_SRC_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets tv_src_ctl to value 0"]
impl crate::Resettable for TV_SRC_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
