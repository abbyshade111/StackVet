//! What the signed-in suite asked the running app, and what it answered, kept so `seen.json` can
//! show it (ADR-082, backlog 0229, part 1).
//!
//! Only the app's own answers are kept, from the requests the suite sends to the app. The test
//! model's and the test sign-in provider's answers are not the app's, and are kept with the other
//! stand-ins' records. A request's headers and body are never kept: they carry the test accounts'
//! passwords and session cookies, which the suite sends and the record must not hold. For an AI question, the prompt
//! `sv` itself wrote is kept beside the answer, handed over by the AI suite through `Http::prompt` (backlog 0623):
//! only that string, never the fields the app's page fills in, and never the request's body.

use super::Http;
use crate::probes::{ProbeRequest, ProbeResponse};

/// One question the signed-in suite asked the app, and its answer if one came.
#[derive(Debug, Clone, PartialEq)]
pub struct Recorded {
    /// The id the check gave its request (`timeout-keep-busy`), which the answer carries too. A
    /// repeated id is numbered (`…-2`), so every answer in one record has its own.
    pub id: String,
    pub method: String,
    pub path: String,
    /// The status the app answered with; `None` when it did not answer.
    pub status: Option<u16>,
    /// The response headers, names lowercased, as the run keeps them.
    pub headers: Vec<(String, String)>,
    /// The start of the response body, as the run keeps it.
    pub body: String,
    /// The prompt `sv` wrote for an AI question asked as this id, when the AI suite handed one over (backlog 0623).
    pub prompt: Option<String>,
}

/// Sends through `inner` as `inner` would, and keeps each exchange with the app.
pub struct Recording<'a> {
    inner: Box<dyn Http + 'a>,
    exchanges: Vec<Recorded>,
    used: std::collections::BTreeSet<String>,
    /// The prompts handed over, by the id their question is sent as.
    prompts: std::collections::BTreeMap<String, String>,
}

impl<'a> Recording<'a> {
    pub fn new(inner: Box<dyn Http + 'a>) -> Self {
        Self {
            inner,
            exchanges: Vec::new(),
            used: std::collections::BTreeSet::new(),
            prompts: std::collections::BTreeMap::new(),
        }
    }

    /// The exchanges kept, in the order they were asked.
    pub fn finish(self) -> Vec<Recorded> {
        self.exchanges
    }

    /// `base`, or `base` numbered from 2 when an earlier answer already has it.
    fn unique_id(&mut self, base: &str) -> String {
        let mut candidate = base.to_owned();
        let mut n = 1;
        while self.used.contains(&candidate) {
            n += 1;
            candidate = format!("{base}-{n}");
        }
        self.used.insert(candidate.clone());
        candidate
    }

    fn keep(&mut self, request: &ProbeRequest, response: Option<&ProbeResponse>) {
        let id = self.unique_id(&request.id);
        // Kept for every answer to its id, so a question asked again after a limiter's wait keeps it too.
        let prompt = self.prompts.get(&request.id).cloned();
        self.exchanges.push(Recorded {
            id,
            method: request.method.clone(),
            path: request.path.clone(),
            status: response.map(|r| r.status),
            headers: response.map(|r| r.headers.clone()).unwrap_or_default(),
            body: response.map(|r| r.body.clone()).unwrap_or_default(),
            prompt,
        });
    }
}

impl Http for Recording<'_> {
    fn send(&mut self, request: &ProbeRequest) -> Option<ProbeResponse> {
        let response = self.inner.send(request);
        self.keep(request, response.as_ref());
        response
    }

    fn send_together(&mut self, requests: &[ProbeRequest]) -> Option<Vec<Option<ProbeResponse>>> {
        let answers = self.inner.send_together(requests)?;
        for (request, answer) in requests.iter().zip(&answers) {
            self.keep(request, answer.as_ref());
        }
        Some(answers)
    }

    fn send_in_turn(&mut self, requests: &[ProbeRequest]) -> Option<Vec<Option<ProbeResponse>>> {
        let answers = self.inner.send_in_turn(requests)?;
        for (request, answer) in requests.iter().zip(&answers) {
            self.keep(request, answer.as_ref());
        }
        Some(answers)
    }

    // The rest is passed through as it is: none of it is the app's own answer.
    fn mail(&mut self, to: &str, at_least: usize) -> Option<Vec<String>> {
        self.inner.mail(to, at_least)
    }

    fn now(&mut self) -> u64 {
        self.inner.now()
    }

    fn wait(&mut self, seconds: u64) {
        self.inner.wait(seconds)
    }

    fn say(&mut self, line: &str) {
        self.inner.say(line)
    }

    fn prompt(&mut self, id: &str, prompt: &str) {
        self.prompts.insert(id.to_owned(), prompt.to_owned());
        self.inner.prompt(id, prompt)
    }

    fn provider(&mut self, request: &ProbeRequest) -> Option<ProbeResponse> {
        self.inner.provider(request)
    }

    fn browser(&mut self, job: &crate::browser::Job) -> Option<Vec<serde_json::Value>> {
        self.inner.browser(job)
    }

    fn model(&mut self, request: &ProbeRequest) -> Option<ProbeResponse> {
        self.inner.model(request)
    }

    fn model_address(&mut self) -> Option<String> {
        self.inner.model_address()
    }
}

#[cfg(test)]
#[path = "recording_tests.rs"]
mod recording_tests;

#[cfg(test)]
#[path = "recording_prompt_tests.rs"]
mod recording_prompt_tests;
