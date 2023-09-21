-- MySQL Workbench Forward Engineering

SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0;
SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0;
SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='ONLY_FULL_GROUP_BY,STRICT_TRANS_TABLES,NO_ZERO_IN_DATE,NO_ZERO_DATE,ERROR_FOR_DIVISION_BY_ZERO,NO_ENGINE_SUBSTITUTION';

-- -----------------------------------------------------
-- Schema 3ways_db
-- -----------------------------------------------------

-- -----------------------------------------------------
-- Schema 3ways_db
-- -----------------------------------------------------
CREATE SCHEMA IF NOT EXISTS `3ways_db` DEFAULT CHARACTER SET utf8 ;
USE `3ways_db` ;

-- -----------------------------------------------------
-- Table `3ways_db`.`users`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `3ways_db`.`users` (
  `name` VARCHAR(45) NOT NULL,
  `password` VARCHAR(64) NOT NULL,
  `email` VARCHAR(64) NOT NULL,
  `phone` VARCHAR(16) NOT NULL DEFAULT '',
  PRIMARY KEY (`name`));


-- -----------------------------------------------------
-- Table `3ways_db`.`commitments`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `3ways_db`.`commitments` (
  `name` VARCHAR(45) NOT NULL,
  `description` TEXT NOT NULL,
  PRIMARY KEY (`name`))
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `3ways_db`.`initiatives`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `3ways_db`.`initiatives` (
  `name` VARCHAR(45) NOT NULL,
  `commitment_name` VARCHAR(45) NOT NULL,
  `description` TEXT NOT NULL,
  PRIMARY KEY (`name`),
  INDEX `fk_initiatives_commitments_idx` (`commitment_name` ASC) VISIBLE,
  CONSTRAINT `fk_initiatives_commitments`
    FOREIGN KEY (`commitment_name`)
    REFERENCES `3ways_db`.`commitments` (`name`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `3ways_db`.`supports`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `3ways_db`.`supports` (
  `user_name` VARCHAR(45) NOT NULL,
  `initiative_name` VARCHAR(45) NOT NULL,
  `adopt_since` TIMESTAMP NULL,
  PRIMARY KEY (`user_name`, `initiative_name`),
  INDEX `fk_supports_initiatives_idx` (`initiative_name` ASC),
  CONSTRAINT `fk_supports_users`
    FOREIGN KEY (`user_name`)
    REFERENCES `3ways_db`.`users` (`name`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `fk_supports_initiatives`
    FOREIGN KEY (`initiative_name`)
    REFERENCES `3ways_db`.`initiatives` (`name`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;


SET SQL_MODE=@OLD_SQL_MODE;
SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS;
SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS;
