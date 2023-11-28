#[doc = "Register `twi_efr` reader"]
pub type R = crate::R<TWI_EFR_SPEC>;
#[doc = "Register `twi_efr` writer"]
pub type W = crate::W<TWI_EFR_SPEC>;
#[doc = "Field `dbn` reader - Data Byte Number Follow Read Command Control"]
pub type DBN_R = crate::FieldReader<DBN_A>;
#[doc = "Data Byte Number Follow Read Command Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DBN_A {
    #[doc = "0: No data byte can be written after the read command"]
    B0 = 0,
    #[doc = "1: 1-byte data can be written after the read command"]
    B1 = 1,
    #[doc = "2: 2-byte data can be written after the read command"]
    B2 = 2,
    #[doc = "3: 3-byte data can be written after the read command"]
    B3 = 3,
}
impl From<DBN_A> for u8 {
    #[inline(always)]
    fn from(variant: DBN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DBN_A {
    type Ux = u8;
}
impl DBN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBN_A {
        match self.bits {
            0 => DBN_A::B0,
            1 => DBN_A::B1,
            2 => DBN_A::B2,
            3 => DBN_A::B3,
            _ => unreachable!(),
        }
    }
    #[doc = "No data byte can be written after the read command"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DBN_A::B0
    }
    #[doc = "1-byte data can be written after the read command"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DBN_A::B1
    }
    #[doc = "2-byte data can be written after the read command"]
    #[inline(always)]
    pub fn is_b2(&self) -> bool {
        *self == DBN_A::B2
    }
    #[doc = "3-byte data can be written after the read command"]
    #[inline(always)]
    pub fn is_b3(&self) -> bool {
        *self == DBN_A::B3
    }
}
#[doc = "Field `dbn` writer - Data Byte Number Follow Read Command Control"]
pub type DBN_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DBN_A>;
impl<'a, REG> DBN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No data byte can be written after the read command"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DBN_A::B0)
    }
    #[doc = "1-byte data can be written after the read command"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DBN_A::B1)
    }
    #[doc = "2-byte data can be written after the read command"]
    #[inline(always)]
    pub fn b2(self) -> &'a mut crate::W<REG> {
        self.variant(DBN_A::B2)
    }
    #[doc = "3-byte data can be written after the read command"]
    #[inline(always)]
    pub fn b3(self) -> &'a mut crate::W<REG> {
        self.variant(DBN_A::B3)
    }
}
impl R {
    #[doc = "Bits 0:1 - Data Byte Number Follow Read Command Control"]
    #[inline(always)]
    pub fn dbn(&self) -> DBN_R {
        DBN_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Data Byte Number Follow Read Command Control"]
    #[inline(always)]
    #[must_use]
    pub fn dbn(&mut self) -> DBN_W<TWI_EFR_SPEC> {
        DBN_W::new(self, 0)
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
#[doc = "TWI Enhance Feature Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twi_efr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twi_efr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWI_EFR_SPEC;
impl crate::RegisterSpec for TWI_EFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`twi_efr::R`](R) reader structure"]
impl crate::Readable for TWI_EFR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`twi_efr::W`](W) writer structure"]
impl crate::Writable for TWI_EFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets twi_efr to value 0"]
impl crate::Resettable for TWI_EFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
