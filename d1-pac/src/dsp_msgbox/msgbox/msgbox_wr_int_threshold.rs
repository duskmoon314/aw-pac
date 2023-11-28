#[doc = "Register `msgbox_wr_int_threshold%s` reader"]
pub type R = crate::R<MSGBOX_WR_INT_THRESHOLD_SPEC>;
#[doc = "Register `msgbox_wr_int_threshold%s` writer"]
pub type W = crate::W<MSGBOX_WR_INT_THRESHOLD_SPEC>;
#[doc = "Field `msg_wr_int_threshold_cfg` reader - Configure the FIFO empty level to trigger the write interrupt for user1"]
pub type MSG_WR_INT_THRESHOLD_CFG_R = crate::FieldReader<MSG_WR_INT_THRESHOLD_CFG_A>;
#[doc = "Configure the FIFO empty level to trigger the write interrupt for user1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSG_WR_INT_THRESHOLD_CFG_A {
    #[doc = "0: `0`"]
    T1 = 0,
    #[doc = "1: `1`"]
    T2 = 1,
    #[doc = "2: `10`"]
    T4 = 2,
    #[doc = "3: `11`"]
    T8 = 3,
}
impl From<MSG_WR_INT_THRESHOLD_CFG_A> for u8 {
    #[inline(always)]
    fn from(variant: MSG_WR_INT_THRESHOLD_CFG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MSG_WR_INT_THRESHOLD_CFG_A {
    type Ux = u8;
}
impl MSG_WR_INT_THRESHOLD_CFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSG_WR_INT_THRESHOLD_CFG_A {
        match self.bits {
            0 => MSG_WR_INT_THRESHOLD_CFG_A::T1,
            1 => MSG_WR_INT_THRESHOLD_CFG_A::T2,
            2 => MSG_WR_INT_THRESHOLD_CFG_A::T4,
            3 => MSG_WR_INT_THRESHOLD_CFG_A::T8,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_t1(&self) -> bool {
        *self == MSG_WR_INT_THRESHOLD_CFG_A::T1
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_t2(&self) -> bool {
        *self == MSG_WR_INT_THRESHOLD_CFG_A::T2
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_t4(&self) -> bool {
        *self == MSG_WR_INT_THRESHOLD_CFG_A::T4
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_t8(&self) -> bool {
        *self == MSG_WR_INT_THRESHOLD_CFG_A::T8
    }
}
#[doc = "Field `msg_wr_int_threshold_cfg` writer - Configure the FIFO empty level to trigger the write interrupt for user1"]
pub type MSG_WR_INT_THRESHOLD_CFG_W<'a, REG> =
    crate::FieldWriterSafe<'a, REG, 2, MSG_WR_INT_THRESHOLD_CFG_A>;
impl<'a, REG> MSG_WR_INT_THRESHOLD_CFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn t1(self) -> &'a mut crate::W<REG> {
        self.variant(MSG_WR_INT_THRESHOLD_CFG_A::T1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn t2(self) -> &'a mut crate::W<REG> {
        self.variant(MSG_WR_INT_THRESHOLD_CFG_A::T2)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn t4(self) -> &'a mut crate::W<REG> {
        self.variant(MSG_WR_INT_THRESHOLD_CFG_A::T4)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn t8(self) -> &'a mut crate::W<REG> {
        self.variant(MSG_WR_INT_THRESHOLD_CFG_A::T8)
    }
}
impl R {
    #[doc = "Bits 0:1 - Configure the FIFO empty level to trigger the write interrupt for user1"]
    #[inline(always)]
    pub fn msg_wr_int_threshold_cfg(&self) -> MSG_WR_INT_THRESHOLD_CFG_R {
        MSG_WR_INT_THRESHOLD_CFG_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Configure the FIFO empty level to trigger the write interrupt for user1"]
    #[inline(always)]
    #[must_use]
    pub fn msg_wr_int_threshold_cfg(
        &mut self,
    ) -> MSG_WR_INT_THRESHOLD_CFG_W<MSGBOX_WR_INT_THRESHOLD_SPEC> {
        MSG_WR_INT_THRESHOLD_CFG_W::new(self, 0)
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
#[doc = "Message Box Write Interrupt Threshold Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msgbox_wr_int_threshold::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msgbox_wr_int_threshold::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSGBOX_WR_INT_THRESHOLD_SPEC;
impl crate::RegisterSpec for MSGBOX_WR_INT_THRESHOLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msgbox_wr_int_threshold::R`](R) reader structure"]
impl crate::Readable for MSGBOX_WR_INT_THRESHOLD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`msgbox_wr_int_threshold::W`](W) writer structure"]
impl crate::Writable for MSGBOX_WR_INT_THRESHOLD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets msgbox_wr_int_threshold%s to value 0"]
impl crate::Resettable for MSGBOX_WR_INT_THRESHOLD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
