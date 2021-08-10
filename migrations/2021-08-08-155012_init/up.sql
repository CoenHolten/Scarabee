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
  `name` VARCHAR(16) NOT NULL,
  `password` VARCHAR(32) NOT NULL,
  `email` VARCHAR(255) NULL,
  `phone` VARCHAR(16) NULL,
  PRIMARY KEY (`name`));


-- -----------------------------------------------------
-- Table `3ways_db`.`groups`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `3ways_db`.`groups` (
  `name` VARCHAR(16) NOT NULL,
  `description` LONGTEXT NOT NULL,
  `commitment` LONGTEXT NULL,
  `is_commitment` TINYINT NOT NULL,
  `is_concept` TINYINT NULL,
  PRIMARY KEY (`name`))
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `3ways_db`.`user_relations`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `3ways_db`.`user_relations` (
  `user` VARCHAR(16) NOT NULL,
  `group` VARCHAR(16) NOT NULL,
  `is_adoption` TINYINT NOT NULL,
  `is_support` TINYINT NOT NULL,
  PRIMARY KEY (`user`, `group`),
  INDEX `group_idx` (`group` ASC) VISIBLE,
  CONSTRAINT `relation_user`
    FOREIGN KEY (`user`)
    REFERENCES `3ways_db`.`users` (`name`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `relation_group`
    FOREIGN KEY (`group`)
    REFERENCES `3ways_db`.`groups` (`name`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `3ways_db`.`group_adoptions`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `3ways_db`.`group_adoptions` (
  `user` VARCHAR(16) NOT NULL,
  `parent_group` VARCHAR(16) NOT NULL,
  `child_group` VARCHAR(16) NOT NULL,
  PRIMARY KEY (`user`, `child_group`, `parent_group`),
  INDEX `parent_group_idx` (`parent_group` ASC) VISIBLE,
  INDEX `child_group_idx` (`child_group` ASC) INVISIBLE,
  CONSTRAINT `adoption_user`
    FOREIGN KEY (`user`)
    REFERENCES `3ways_db`.`users` (`name`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `adoption_parent`
    FOREIGN KEY (`parent_group`)
    REFERENCES `3ways_db`.`groups` (`name`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `adoption_child`
    FOREIGN KEY (`child_group`)
    REFERENCES `3ways_db`.`groups` (`name`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;


SET SQL_MODE=@OLD_SQL_MODE;
SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS;
SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS;
