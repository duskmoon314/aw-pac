#[doc = "Register `tv_ctl` reader"]
pub struct R(crate::R<TV_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TV_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TV_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TV_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tv_ctl` writer"]
pub struct W(crate::W<TV_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TV_CTL_SPEC>;
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
impl From<crate::W<TV_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TV_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tv_src_sel` reader - TV Source Select\n\nNote: The priority of this bit is higher than TV_SRC_SEL(bit\\[2:0\\]) in TV_SRC_CTL_REG."]
pub type TV_SRC_SEL_R = crate::BitReader<TV_SRC_SEL_A>;
#[doc = "TV Source Select\n\nNote: The priority of this bit is higher than TV_SRC_SEL(bit\\[2:0\\]) in TV_SRC_CTL_REG.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TV_SRC_SEL_A {
    #[doc = "1: BLUE data"]
    BLUE_DATA = 1,
}
impl From<TV_SRC_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: TV_SRC_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl TV_SRC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TV_SRC_SEL_A> {
        match self.bits {
            true => Some(TV_SRC_SEL_A::BLUE_DATA),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BLUE_DATA`"]
    #[inline(always)]
    pub fn is_blue_data(&self) -> bool {
        *self == TV_SRC_SEL_A::BLUE_DATA
    }
}
#[doc = "Field `tv_src_sel` writer - TV Source Select\n\nNote: The priority of this bit is higher than TV_SRC_SEL(bit\\[2:0\\]) in TV_SRC_CTL_REG."]
pub type TV_SRC_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TV_CTL_SPEC, TV_SRC_SEL_A, O>;
impl<'a, const O: u8> TV_SRC_SEL_W<'a, O> {
    #[doc = "BLUE data"]
    #[inline(always)]
    pub fn blue_data(self) -> &'a mut W {
        self.variant(TV_SRC_SEL_A::BLUE_DATA)
    }
}
#[doc = "Field `start_delay` reader - This is for DE0 and DE1."]
pub type START_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `start_delay` writer - This is for DE0 and DE1."]
pub type START_DELAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TV_CTL_SPEC, u8, u8, 5, O>;
#[doc = "Field `tv_en` reader - When enable TCON_TV, this bit and the 0x0000\\[bit31\\] need to be enabled."]
pub type TV_EN_R = crate::BitReader<TV_EN_A>;
#[doc = "When enable TCON_TV, this bit and the 0x0000\\[bit31\\] need to be enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TV_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<TV_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TV_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TV_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TV_EN_A {
        match self.bits {
            false => TV_EN_A::DISABLE,
            true => TV_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TV_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TV_EN_A::ENABLE
    }
}
#[doc = "Field `tv_en` writer - When enable TCON_TV, this bit and the 0x0000\\[bit31\\] need to be enabled."]
pub type TV_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TV_CTL_SPEC, TV_EN_A, O>;
impl<'a, const O: u8> TV_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TV_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TV_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 1 - TV Source Select\n\nNote: The priority of this bit is higher than TV_SRC_SEL(bit\\[2:0\\]) in TV_SRC_CTL_REG."]
    #[inline(always)]
    pub fn tv_src_sel(&self) -> TV_SRC_SEL_R {
        TV_SRC_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:8 - This is for DE0 and DE1."]
    #[inline(always)]
    pub fn start_delay(&self) -> START_DELAY_R {
        START_DELAY_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - When enable TCON_TV, this bit and the 0x0000\\[bit31\\] need to be enabled."]
    #[inline(always)]
    pub fn tv_en(&self) -> TV_EN_R {
        TV_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - TV Source Select\n\nNote: The priority of this bit is higher than TV_SRC_SEL(bit\\[2:0\\]) in TV_SRC_CTL_REG."]
    #[inline(always)]
    #[must_use]
    pub fn tv_src_sel(&mut self) -> TV_SRC_SEL_W<1> {
        TV_SRC_SEL_W::new(self)
    }
    #[doc = "Bits 4:8 - This is for DE0 and DE1."]
    #[inline(always)]
    #[must_use]
    pub fn start_delay(&mut self) -> START_DELAY_W<4> {
        START_DELAY_W::new(self)
    }
    #[doc = "Bit 31 - When enable TCON_TV, this bit and the 0x0000\\[bit31\\] need to be enabled."]
    #[inline(always)]
    #[must_use]
    pub fn tv_en(&mut self) -> TV_EN_W<31> {
        TV_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tv_ctl](index.html) module"]
pub struct TV_CTL_SPEC;
impl crate::RegisterSpec for TV_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tv_ctl::R](R) reader structure"]
impl crate::Readable for TV_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tv_ctl::W](W) writer structure"]
impl crate::Writable for TV_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tv_ctl to value 0"]
impl crate::Resettable for TV_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
