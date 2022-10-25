#[doc = "Register `asrcmbiststat` reader"]
pub struct R(crate::R<ASRCMBISTSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASRCMBISTSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASRCMBISTSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASRCMBISTSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `asrcmbiststat` writer"]
pub struct W(crate::W<ASRCMBISTSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASRCMBISTSTAT_SPEC>;
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
impl From<crate::W<ASRCMBISTSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASRCMBISTSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ram_busy_status` reader - RAM busy status"]
pub type RAM_BUSY_STATUS_R = crate::BitReader<RAM_BUSY_STATUS_A>;
#[doc = "RAM busy status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAM_BUSY_STATUS_A {
    #[doc = "0: `0`"]
    IDLE = 0,
    #[doc = "1: `1`"]
    BUSY = 1,
}
impl From<RAM_BUSY_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_BUSY_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl RAM_BUSY_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_BUSY_STATUS_A {
        match self.bits {
            false => RAM_BUSY_STATUS_A::IDLE,
            true => RAM_BUSY_STATUS_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == RAM_BUSY_STATUS_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == RAM_BUSY_STATUS_A::BUSY
    }
}
#[doc = "Field `ram_stop_status` reader - RAM stop status"]
pub type RAM_STOP_STATUS_R = crate::BitReader<RAM_STOP_STATUS_A>;
#[doc = "RAM stop status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAM_STOP_STATUS_A {
    #[doc = "0: `0`"]
    RUNNING = 0,
    #[doc = "1: `1`"]
    STOP = 1,
}
impl From<RAM_STOP_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_STOP_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl RAM_STOP_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_STOP_STATUS_A {
        match self.bits {
            false => RAM_STOP_STATUS_A::RUNNING,
            true => RAM_STOP_STATUS_A::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `RUNNING`"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == RAM_STOP_STATUS_A::RUNNING
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == RAM_STOP_STATUS_A::STOP
    }
}
#[doc = "Field `ram_bist_error_cycle` reader - RAM BIST error cycle"]
pub type RAM_BIST_ERROR_CYCLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ram_bist_error_pattern` reader - RAM BIST error pattern"]
pub type RAM_BIST_ERROR_PATTERN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ram_bist_err_status` reader - RAM BIST error status"]
pub type RAM_BIST_ERR_STATUS_R = crate::BitReader<RAM_BIST_ERR_STATUS_A>;
#[doc = "RAM BIST error status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAM_BIST_ERR_STATUS_A {
    #[doc = "0: `0`"]
    NO_EFFECT = 0,
    #[doc = "1: `1`"]
    ERROR = 1,
}
impl From<RAM_BIST_ERR_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_BIST_ERR_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl RAM_BIST_ERR_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_BIST_ERR_STATUS_A {
        match self.bits {
            false => RAM_BIST_ERR_STATUS_A::NO_EFFECT,
            true => RAM_BIST_ERR_STATUS_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == RAM_BIST_ERR_STATUS_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == RAM_BIST_ERR_STATUS_A::ERROR
    }
}
#[doc = "Field `rom_busy_status` reader - ROM busy status"]
pub type ROM_BUSY_STATUS_R = crate::BitReader<ROM_BUSY_STATUS_A>;
#[doc = "ROM busy status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROM_BUSY_STATUS_A {
    #[doc = "0: `0`"]
    IDLE = 0,
    #[doc = "1: `1`"]
    BUSY = 1,
}
impl From<ROM_BUSY_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: ROM_BUSY_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl ROM_BUSY_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROM_BUSY_STATUS_A {
        match self.bits {
            false => ROM_BUSY_STATUS_A::IDLE,
            true => ROM_BUSY_STATUS_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == ROM_BUSY_STATUS_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == ROM_BUSY_STATUS_A::BUSY
    }
}
#[doc = "Field `rom_bist_error_sum` reader - ROM BIST error sum"]
pub type ROM_BIST_ERROR_SUM_R = crate::BitReader<bool>;
#[doc = "Field `rom_bist_error_xor` reader - ROM BIST error xor"]
pub type ROM_BIST_ERROR_XOR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - RAM busy status"]
    #[inline(always)]
    pub fn ram_busy_status(&self) -> RAM_BUSY_STATUS_R {
        RAM_BUSY_STATUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RAM stop status"]
    #[inline(always)]
    pub fn ram_stop_status(&self) -> RAM_STOP_STATUS_R {
        RAM_STOP_STATUS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - RAM BIST error cycle"]
    #[inline(always)]
    pub fn ram_bist_error_cycle(&self) -> RAM_BIST_ERROR_CYCLE_R {
        RAM_BIST_ERROR_CYCLE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - RAM BIST error pattern"]
    #[inline(always)]
    pub fn ram_bist_error_pattern(&self) -> RAM_BIST_ERROR_PATTERN_R {
        RAM_BIST_ERROR_PATTERN_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - RAM BIST error status"]
    #[inline(always)]
    pub fn ram_bist_err_status(&self) -> RAM_BIST_ERR_STATUS_R {
        RAM_BIST_ERR_STATUS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - ROM busy status"]
    #[inline(always)]
    pub fn rom_busy_status(&self) -> ROM_BUSY_STATUS_R {
        ROM_BUSY_STATUS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ROM BIST error sum"]
    #[inline(always)]
    pub fn rom_bist_error_sum(&self) -> ROM_BIST_ERROR_SUM_R {
        ROM_BIST_ERROR_SUM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ROM BIST error xor"]
    #[inline(always)]
    pub fn rom_bist_error_xor(&self) -> ROM_BIST_ERROR_XOR_R {
        ROM_BIST_ERROR_XOR_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ASRC MBIST Test Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asrcmbiststat](index.html) module"]
pub struct ASRCMBISTSTAT_SPEC;
impl crate::RegisterSpec for ASRCMBISTSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [asrcmbiststat::R](R) reader structure"]
impl crate::Readable for ASRCMBISTSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [asrcmbiststat::W](W) writer structure"]
impl crate::Writable for ASRCMBISTSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets asrcmbiststat to value 0"]
impl crate::Resettable for ASRCMBISTSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
