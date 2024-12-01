// Template is taken from https://github.com/stuxf/basic-typst-resume-template

#let resume(
  author: "",
  pronouns: "",
  location: "",
  email: "",
  github: "",
  linkedin: "",
  phone: "",
  personal-site: "",
  orcid: "",
  accent-color: "#000000",
  font: "New Computer Modern",
  body,
) = {

  // Sets document metadata
  set document(author: author, title: author)

  // Document-wide formatting, including font and margins
  set text(
    // LaTeX style font
    font: font,
    size: 10pt,
    lang: "en",
    // Disable ligatures so ATS systems do not get confused when parsing fonts.
    ligatures: false
  )

  // Reccomended to have 0.5in margin on all sides
  set page(
    margin: (0.5in),
    "us-letter",
  )

  // Link styles
  show link: underline


  // Small caps for section titles
  show heading.where(level: 2): it => [
    #pad(top: 0pt, bottom: -10pt, [#smallcaps(it.body)])
    #line(length: 100%, stroke: 1pt)
  ]

  // Accent Color Styling
  show heading: set text(
    fill: rgb(accent-color),
  )

  show link: set text(
    fill: rgb(accent-color),
  )

  // Name will be aligned left, bold and big
  show heading.where(level: 1): it => [
    #set align(left)
    #set text(
      weight: 700,
      size: 20pt,
    )
    #pad(it.body)
  ]

  // Level 1 Heading
  [= #(author)]

  // Personal Info Helper
  let contact-item(value, prefix: "", link-type: "") = {
    if value != "" {
      if link-type != "" {
        link(link-type + value)[#(prefix + value)]
      } else {
        value
      }
    }
  }

  // Personal Info
  pad(
    top: 0.25em,
    align(left)[
      #{
        let items = (
          contact-item(pronouns),
          contact-item(phone),
          contact-item(location),
          contact-item(email, link-type: "mailto:"),
          contact-item(github, link-type: "https://"),
          contact-item(linkedin, link-type: "https://"),
          contact-item(personal-site, link-type: "https://"),
          contact-item(orcid, link-type: "https://orcid.org/"),
        )
        items.filter(x => x != none).join("  |  ")
      }
    ],
  )

  // Main body.
  set par(justify: true)

  body
}

// Generic two by two component for resume
#let generic-two-by-two(
  top-left: "",
  top-right: "",
  bottom-left: "",
  bottom-right: "",
) = {
  [
    #top-left #h(1fr) #top-right \
    #bottom-left #h(1fr) #bottom-right
  ]
}

// Generic one by two component for resume
#let generic-one-by-two(
  left: "",
  right: "",
) = {
  [
    #left #h(1fr) #right
  ]
}

// Cannot just use normal --- ligature becuase ligatures are disabled for good reasons
#let dates-helper(
  start-date: "",
  end-date: "",
) = {
  start-date + " " + $dash.em$ + " " + end-date
}

// Section components below
#let edu(
  institution: "",
  dates: "",
  degree: "",
  gpa: "",
  location: "",
) = {
  generic-two-by-two(
    top-left: strong(institution),
    top-right: location,
    bottom-left: emph(degree),
    bottom-right: emph(dates),
  )
}

#let work(
  title: "",
  dates: "",
  company: "",
  location: "",
) = {
  generic-two-by-two(
    top-left: strong(title),
    top-right: dates,
    bottom-left: company,
    bottom-right: emph(location),
  )
}

#let project(
  role: "",
  name: "",
  url: "",
  dates: "",
) = {
  generic-one-by-two(
    left: {
      if role == "" {
        [*#name* #if url != "" and dates != "" [ (#link("https://" + url)[#url])]]
      } else {
        [*#role*, #name #if url != "" and dates != ""  [ (#link("https://" + url)[#url])]]
      }
    },
    right: {
      if dates == "" and url != "" {
        link("https://" + url)[#url]
      } else {
        dates
      }
    },
  )
}

#let certificates(
  name: "",
  issuer: "",
  url: "",
  date: "",
) = {
  [
    *#name*, #issuer
    #if url != "" {
      [ (#link("https://" + url)[#url])]
    }
    #h(1fr) #date
  ]
}

#let extracurriculars(
  activity: "",
  dates: "",
) = {
  generic-one-by-two(
    left: strong(activity),
    right: dates,
  )
}

// Put your personal information here, replacing mine
#let name = "Nomen Nescio"
#let location = "Amsterdam, Netherlands"
#let email = "loremipsum@dolor.com"
#let github = "github.com/lorem"
#let linkedin = "linkedin.com/in/lorem"
#let phone = "+31 6 12345678"
#let personal-site = "lorem.com"

#show: resume.with(
  author: name,
  // All the lines below are optional.
  // For example, if you want to to hide your phone number:
  // feel free to comment those lines out and they will not show.
  location: location,
  email: email,
  github: github,
  linkedin: linkedin,
  phone: phone,
  personal-site: personal-site,
  accent-color: "#26428b",
  font: "New Computer Modern",
)

/*
* Lines that start with == are formatted into section headings
* You can use the specific formatting functions if needed
* The following formatting functions are listed below
* #edu(dates: "", degree: "", gpa: "", institution: "", location: "")
* #work(company: "", dates: "", location: "", title: "")
* #project(dates: "", name: "", role: "", url: "")
* certificates(name: "", issuer: "", url: "", date: "")
* #extracurriculars(activity: "", dates: "")
* There are also the following generic functions that don't apply any formatting
* #generic-two-by-two(top-left: "", top-right: "", bottom-left: "", bottom-right: "")
* #generic-one-by-two(left: "", right: "")
*/

== Education
#edu(
  institution: "Universiteit van Lorem",
  location: "Amsterdam, Netherlands",
  dates: dates-helper(start-date: "Sep 2023", end-date: "Jul 2027"),
  degree: "Bachelor of Science, Computer Science",
)
- Lorem ipsum dolor sit amet, consectetur adipiscing elit
- Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua
- Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris

== Work Experience
#work(
  title: "Software Development Engineer",
  location: "Amsterdam, Netherlands",
  company: "Lorem Technologies BV",
  dates: dates-helper(start-date: "May 2024", end-date: "Present"),
)
- Lorem ipsum dolor sit amet, consectetur adipiscing elit, reducing costs by 45%
- Sed do eiusmod tempor incididunt ut labore with team of 6 developers
- Ut enim ad minim veniam, quis nostrud exercitation results in 65% improvement

#work(
  title: "Backend Developer",
  location: "Rotterdam, Netherlands",
  company: "Ipsum Solutions NV",
  dates: dates-helper(start-date: "Dec 2023", end-date: "Mar 2024"),
)
- Consectetur adipiscing elit for 15,000+ daily active users
- Implemented ut labore et dolore magna aliqua security system
- Duis aute irure dolor in reprehenderit optimization by 40%



== Projects
#project(
  name: "Dolor",
  role: "Lorem Developer",
  dates: dates-helper(start-date: "Nov 2023", end-date: "Present"),
  url: "lorem.nl",
)
- Lorem ipsum dolor sit amet using React and Node.js, serving 5000+ users
- Duis aute irure dolor in reprehenderit pipeline development
- Excepteur sint occaecat cupidatat non proident deployment system

== Extracurricular Activities
#extracurriculars(
  activity: "Dutch Developer Community",
  dates: dates-helper(start-date: "Jan 2023", end-date: "Present"),
)
- Lorem ipsum dolor sit amet, ranked #5 nationally
- Organized tempor incididunt events with 500+ participants
- Mentored 12+ junior developers in ut labore et dolore

== Skills
- *Programming Languages*: Python, Rust, Bash, Java
- *Technologies*: Git, Kubernetes, Harness, Jenkins