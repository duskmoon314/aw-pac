#[doc = "Register `ce_esr` reader"]
pub type R = crate::R<CE_ESR_SPEC>;
#[doc = "Register `ce_esr` writer"]
pub type W = crate::W<CE_ESR_SPEC>;
#[doc = "Field `task_channel_error_type[0-3]` reader - Task Channel Error Type"]
pub type TASK_CHANNEL_ERROR_TYPE_R = crate::FieldReader<TASK_CHANNEL_ERROR_TYPE_A>;
#[doc = "Task Channel Error Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TASK_CHANNEL_ERROR_TYPE_A {
    #[doc = "1: Algorithm not support"]
    ALGORITHM_NOT_SUPPORT = 1,
    #[doc = "2: Data length error"]
    DATA_LENGTH_ERROR = 2,
    #[doc = "4: keysram access error for AES"]
    KEYSRAM_ACCESS_ERROR = 4,
}
impl From<TASK_CHANNEL_ERROR_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: TASK_CHANNEL_ERROR_TYPE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TASK_CHANNEL_ERROR_TYPE_A {
    type Ux = u8;
}
impl TASK_CHANNEL_ERROR_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TASK_CHANNEL_ERROR_TYPE_A> {
        match self.bits {
            1 => Some(TASK_CHANNEL_ERROR_TYPE_A::ALGORITHM_NOT_SUPPORT),
            2 => Some(TASK_CHANNEL_ERROR_TYPE_A::DATA_LENGTH_ERROR),
            4 => Some(TASK_CHANNEL_ERROR_TYPE_A::KEYSRAM_ACCESS_ERROR),
            _ => None,
        }
    }
    #[doc = "Algorithm not support"]
    #[inline(always)]
    pub fn is_algorithm_not_support(&self) -> bool {
        *self == TASK_CHANNEL_ERROR_TYPE_A::ALGORITHM_NOT_SUPPORT
    }
    #[doc = "Data length error"]
    #[inline(always)]
    pub fn is_data_length_error(&self) -> bool {
        *self == TASK_CHANNEL_ERROR_TYPE_A::DATA_LENGTH_ERROR
    }
    #[doc = "keysram access error for AES"]
    #[inline(always)]
    pub fn is_keysram_access_error(&self) -> bool {
        *self == TASK_CHANNEL_ERROR_TYPE_A::KEYSRAM_ACCESS_ERROR
    }
}
#[doc = "Field `task_channel_error_type[0-3]` writer - Task Channel Error Type"]
pub type TASK_CHANNEL_ERROR_TYPE_W<'a, REG> =
    crate::FieldWriter<'a, REG, 4, TASK_CHANNEL_ERROR_TYPE_A>;
impl<'a, REG> TASK_CHANNEL_ERROR_TYPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Algorithm not support"]
    #[inline(always)]
    pub fn algorithm_not_support(self) -> &'a mut crate::W<REG> {
        self.variant(TASK_CHANNEL_ERROR_TYPE_A::ALGORITHM_NOT_SUPPORT)
    }
    #[doc = "Data length error"]
    #[inline(always)]
    pub fn data_length_error(self) -> &'a mut crate::W<REG> {
        self.variant(TASK_CHANNEL_ERROR_TYPE_A::DATA_LENGTH_ERROR)
    }
    #[doc = "keysram access error for AES"]
    #[inline(always)]
    pub fn keysram_access_error(self) -> &'a mut crate::W<REG> {
        self.variant(TASK_CHANNEL_ERROR_TYPE_A::KEYSRAM_ACCESS_ERROR)
    }
}
impl R {
    #[doc = "Task Channel Error Type\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `task_channel0_error_type` field"]
    #[inline(always)]
    pub fn task_channel_error_type(&self, n: u8) -> TASK_CHANNEL_ERROR_TYPE_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        TASK_CHANNEL_ERROR_TYPE_R::new(((self.bits >> (n * 4)) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Task Channel Error Type"]
    #[inline(always)]
    pub fn task_channel0_error_type(&self) -> TASK_CHANNEL_ERROR_TYPE_R {
        TASK_CHANNEL_ERROR_TYPE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Task Channel Error Type"]
    #[inline(always)]
    pub fn task_channel1_error_type(&self) -> TASK_CHANNEL_ERROR_TYPE_R {
        TASK_CHANNEL_ERROR_TYPE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Task Channel Error Type"]
    #[inline(always)]
    pub fn task_channel2_error_type(&self) -> TASK_CHANNEL_ERROR_TYPE_R {
        TASK_CHANNEL_ERROR_TYPE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Task Channel Error Type"]
    #[inline(always)]
    pub fn task_channel3_error_type(&self) -> TASK_CHANNEL_ERROR_TYPE_R {
        TASK_CHANNEL_ERROR_TYPE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Task Channel Error Type\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `task_channel0_error_type` field"]
    #[inline(always)]
    #[must_use]
    pub fn task_channel_error_type(&mut self, n: u8) -> TASK_CHANNEL_ERROR_TYPE_W<CE_ESR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        TASK_CHANNEL_ERROR_TYPE_W::new(self, n * 4)
    }
    #[doc = "Bits 0:3 - Task Channel Error Type"]
    #[inline(always)]
    #[must_use]
    pub fn task_channel0_error_type(&mut self) -> TASK_CHANNEL_ERROR_TYPE_W<CE_ESR_SPEC> {
        TASK_CHANNEL_ERROR_TYPE_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Task Channel Error Type"]
    #[inline(always)]
    #[must_use]
    pub fn task_channel1_error_type(&mut self) -> TASK_CHANNEL_ERROR_TYPE_W<CE_ESR_SPEC> {
        TASK_CHANNEL_ERROR_TYPE_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Task Channel Error Type"]
    #[inline(always)]
    #[must_use]
    pub fn task_channel2_error_type(&mut self) -> TASK_CHANNEL_ERROR_TYPE_W<CE_ESR_SPEC> {
        TASK_CHANNEL_ERROR_TYPE_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Task Channel Error Type"]
    #[inline(always)]
    #[must_use]
    pub fn task_channel3_error_type(&mut self) -> TASK_CHANNEL_ERROR_TYPE_W<CE_ESR_SPEC> {
        TASK_CHANNEL_ERROR_TYPE_W::new(self, 12)
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
#[doc = "Error Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ce_esr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ce_esr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CE_ESR_SPEC;
impl crate::RegisterSpec for CE_ESR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ce_esr::R`](R) reader structure"]
impl crate::Readable for CE_ESR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ce_esr::W`](W) writer structure"]
impl crate::Writable for CE_ESR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0f;
}
#[doc = "`reset()` method sets ce_esr to value 0"]
impl crate::Resettable for CE_ESR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
