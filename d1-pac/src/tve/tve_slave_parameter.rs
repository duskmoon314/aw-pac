#[doc = "Register `tve_slave_parameter` reader"]
pub struct R(crate::R<TVE_SLAVE_PARAMETER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVE_SLAVE_PARAMETER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVE_SLAVE_PARAMETER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVE_SLAVE_PARAMETER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tve_slave_parameter` writer"]
pub struct W(crate::W<TVE_SLAVE_PARAMETER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVE_SLAVE_PARAMETER_SPEC>;
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
impl From<crate::W<TVE_SLAVE_PARAMETER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVE_SLAVE_PARAMETER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `slave_mode` reader - Slave mode selection\n\nThis bit selects whether the Video Encoder is sync slave, partial slave or sync master. It should be set to B'0'."]
pub type SLAVE_MODE_R = crate::BitReader<SLAVE_MODE_A>;
#[doc = "Slave mode selection\n\nThis bit selects whether the Video Encoder is sync slave, partial slave or sync master. It should be set to B'0'.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLAVE_MODE_A {
    #[doc = "0: The Video Encoder is not a full sync slave (i.e. it is a partial sync slave or a sync master)"]
    NOT_FULL_SYNC_SLAVE = 0,
}
impl From<SLAVE_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SLAVE_MODE_A> {
        match self.bits {
            false => Some(SLAVE_MODE_A::NOT_FULL_SYNC_SLAVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_FULL_SYNC_SLAVE`"]
    #[inline(always)]
    pub fn is_not_full_sync_slave(&self) -> bool {
        *self == SLAVE_MODE_A::NOT_FULL_SYNC_SLAVE
    }
}
#[doc = "Field `slave_mode` writer - Slave mode selection\n\nThis bit selects whether the Video Encoder is sync slave, partial slave or sync master. It should be set to B'0'."]
pub type SLAVE_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TVE_SLAVE_PARAMETER_SPEC, SLAVE_MODE_A, O>;
impl<'a, const O: u8> SLAVE_MODE_W<'a, O> {
    #[doc = "The Video Encoder is not a full sync slave (i.e. it is a partial sync slave or a sync master)"]
    #[inline(always)]
    pub fn not_full_sync_slave(self) -> &'a mut W {
        self.variant(SLAVE_MODE_A::NOT_FULL_SYNC_SLAVE)
    }
}
#[doc = "Field `slave_thresh` reader - Horizontal line adjustment threshold selection\n\nThis bit selects whether the number of lines after which the Video Encoder starts the horizontal line length adjustment is slave mode is 0 or 30."]
pub type SLAVE_THRESH_R = crate::BitReader<SLAVE_THRESH_A>;
#[doc = "Horizontal line adjustment threshold selection\n\nThis bit selects whether the number of lines after which the Video Encoder starts the horizontal line length adjustment is slave mode is 0 or 30.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLAVE_THRESH_A {
    #[doc = "0: Number of lines is 0"]
    _0 = 0,
    #[doc = "1: Number of lines is 30"]
    _30 = 1,
}
impl From<SLAVE_THRESH_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE_THRESH_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE_THRESH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE_THRESH_A {
        match self.bits {
            false => SLAVE_THRESH_A::_0,
            true => SLAVE_THRESH_A::_30,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLAVE_THRESH_A::_0
    }
    #[doc = "Checks if the value of the field is `_30`"]
    #[inline(always)]
    pub fn is_30(&self) -> bool {
        *self == SLAVE_THRESH_A::_30
    }
}
#[doc = "Field `slave_thresh` writer - Horizontal line adjustment threshold selection\n\nThis bit selects whether the number of lines after which the Video Encoder starts the horizontal line length adjustment is slave mode is 0 or 30."]
pub type SLAVE_THRESH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TVE_SLAVE_PARAMETER_SPEC, SLAVE_THRESH_A, O>;
impl<'a, const O: u8> SLAVE_THRESH_W<'a, O> {
    #[doc = "Number of lines is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLAVE_THRESH_A::_0)
    }
    #[doc = "Number of lines is 30"]
    #[inline(always)]
    pub fn _30(self) -> &'a mut W {
        self.variant(SLAVE_THRESH_A::_30)
    }
}
impl R {
    #[doc = "Bit 0 - Slave mode selection\n\nThis bit selects whether the Video Encoder is sync slave, partial slave or sync master. It should be set to B'0'."]
    #[inline(always)]
    pub fn slave_mode(&self) -> SLAVE_MODE_R {
        SLAVE_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Horizontal line adjustment threshold selection\n\nThis bit selects whether the number of lines after which the Video Encoder starts the horizontal line length adjustment is slave mode is 0 or 30."]
    #[inline(always)]
    pub fn slave_thresh(&self) -> SLAVE_THRESH_R {
        SLAVE_THRESH_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slave mode selection\n\nThis bit selects whether the Video Encoder is sync slave, partial slave or sync master. It should be set to B'0'."]
    #[inline(always)]
    #[must_use]
    pub fn slave_mode(&mut self) -> SLAVE_MODE_W<0> {
        SLAVE_MODE_W::new(self)
    }
    #[doc = "Bit 8 - Horizontal line adjustment threshold selection\n\nThis bit selects whether the number of lines after which the Video Encoder starts the horizontal line length adjustment is slave mode is 0 or 30."]
    #[inline(always)]
    #[must_use]
    pub fn slave_thresh(&mut self) -> SLAVE_THRESH_W<8> {
        SLAVE_THRESH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Encoder Slave Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tve_slave_parameter](index.html) module"]
pub struct TVE_SLAVE_PARAMETER_SPEC;
impl crate::RegisterSpec for TVE_SLAVE_PARAMETER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tve_slave_parameter::R](R) reader structure"]
impl crate::Readable for TVE_SLAVE_PARAMETER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tve_slave_parameter::W](W) writer structure"]
impl crate::Writable for TVE_SLAVE_PARAMETER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_slave_parameter to value 0"]
impl crate::Resettable for TVE_SLAVE_PARAMETER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
