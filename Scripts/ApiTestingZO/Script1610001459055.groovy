import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Cookie as Cookie
import groovy.json.JsonSlurper as JsonSlurper

response1 = WS.sendRequest(findTestObject('login', [('url') : GlobalVariable.base_url]))

def slurper = new JsonSlurper()

def result = slurper.parseText(response1.getResponseBodyContent())

def access_token = result.access_token

GlobalVariable.access_token = access_token

println('access_token ' + GlobalVariable.access_token)

response2 = WS.sendRequest(findTestObject('CreateNewCompany', [('url') : GlobalVariable.base_url, ('access_token') : GlobalVariable.access_token]))

def result2 = slurper.parseText(response2.getResponseBodyContent())

def slug = result2.company.slug

GlobalVariable.slug = slug

println('slug' + GlobalVariable.slug)

response3 = WS.sendRequest(findTestObject('Company Registrations/get_user_companies', [('url') : GlobalVariable.base_url
            , ('slug') : GlobalVariable.slug, ('access_token') : GlobalVariable.access_token]))

def result3 = slurper.parseText(response3.getResponseBodyContent())

def get_user_companies = result3.results[0]

println('user companies' +get_user_companies)

